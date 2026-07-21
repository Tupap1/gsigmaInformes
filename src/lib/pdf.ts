import pdfMake from 'pdfmake/build/pdfmake';
import * as pdfFonts from 'pdfmake/build/vfs_fonts';
import { save } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';
import type { CompraAcumulada, ResumenCaja } from './api';

// Set the virtual file system for pdfmake fonts
(pdfMake as any).vfs = (pdfFonts as any).pdfMake?.vfs || (pdfFonts as any).vfs || (pdfMake as any).vfs;

// Registrar Courier y Roboto para evitar errores de compilación por falta de fuentes en vfs
(pdfMake as any).fonts = {
  Courier: {
    normal: 'Courier',
    bold: 'Courier-Bold',
    italics: 'Courier-Oblique',
    bolditalics: 'Courier-BoldOblique'
  },
  Roboto: {
    normal: 'Roboto-Regular.ttf',
    bold: 'Roboto-Medium.ttf',
    italics: 'Roboto-Italic.ttf',
    bolditalics: 'Roboto-MediumItalic.ttf'
  }
};

// Pad utilities for aligning columns in Courier
function padRight(str: string, len: number, padChar = ' '): string {
  if (str.length >= len) return str.slice(0, len);
  return str + padChar.repeat(len - str.length);
}

function padLeft(str: string, len: number, padChar = ' '): string {
  if (str.length >= len) return str.slice(str.length - len);
  return padChar.repeat(len - str.length) + str;
}

function formatCurrency(val: number): string {
  return val.toLocaleString('es-CO', { minimumFractionDigits: 0, maximumFractionDigits: 0 });
}

function formatNumber(val: number): string {
  return val.toLocaleString('es-CO', { minimumFractionDigits: 2, maximumFractionDigits: 2 });
}

export async function generateReportPDF(
  startDate: string,
  endDate: string,
  compras: CompraAcumulada[],
  resumen: ResumenCaja
) {
  const lines: string[] = [];
  const charsPerLine = 48;

  // Formatear cabecera
  // Pág. 1        COMPRAS ACUMULADAS        08-07-2026
  const dateStr = startDate === endDate ? startDate : `${startDate}`;
  const pagPart = "Pág.        1";
  const titlePart = "COMPRAS ACUMULADAS";
  const datePart = dateStr;
  
  // Calcular espacios
  // pagPart (13) + titlePart (18) = 31.
  // Quedan 17 caracteres para alinear la fecha a la derecha
  const headerLine = pagPart + " ".repeat(4) + titlePart + padLeft(datePart, charsPerLine - 31 - 4);
  lines.push(headerLine);
  
  // Línea 2 de cabecera: Nombre de empresa centrado
  const empTitle = "RECICLADORA BOYACA";
  const empLine = " ".repeat((charsPerLine - empTitle.length) / 2) + empTitle;
  lines.push(empLine);
  
  // Rango de fechas si es diferente
  if (startDate !== endDate) {
    const rangeText = `PERIODO: DESDE ${startDate} HASTA ${endDate}`;
    const rangeLine = " ".repeat((charsPerLine - rangeText.length) / 2) + rangeText;
    lines.push(rangeLine);
  }

  lines.push("-".repeat(charsPerLine));
  
  // Column headers
  // PAS (3) + " " (1) + MATERIAL (15) + " " (1) + CANT (8) + " " (1) + TOTAL (11) + " " (1) + PROM (7) = 48
  const tableHeader = "PAS  " + padRight("MATERIAL", 15) + " " + padLeft("Cantidad", 8) + " " + padLeft("Total", 11) + " " + padLeft("Promedio", 7);
  lines.push(tableHeader);
  lines.push("-".repeat(charsPerLine));

  // Items
  let totalCantidad = 0;
  let totalDinero = 0;

  for (const item of compras) {
    totalCantidad += item.cantidad;
    totalDinero += item.total;

    const pas = padRight(item.pas, 3);
    const material = padRight(item.nombre.toUpperCase(), 15);
    const cant = padLeft(formatNumber(item.cantidad), 8);
    const total = padLeft(formatCurrency(item.total), 11);
    const prom = padLeft(formatCurrency(item.costoPromedio), 7);

    lines.push(`${pas}  ${material} ${cant} ${total} ${prom}`);
  }

  lines.push("-".repeat(charsPerLine));

  // Totales de la tabla
  // TOTAL............................  4,151.35     10,351,796.24
  const totalLabel = padRight("TOTAL", 20, '.');
  const totalCantStr = padLeft(formatNumber(totalCantidad), 8);
  const totalDineroStr = padLeft(formatCurrency(totalDinero), 11);
  const totalLine = totalLabel + " " + totalCantStr + " " + totalDineroStr + " ".repeat(8);
  lines.push(totalLine);
  lines.push("");

  // Resumen de caja al pie (exactamente como en la imagen)
  // Label (27) + Op (5) + Value (16) = 48
  lines.push(padRight("Total Base Caja:", 27) + padRight("(+)", 5) + padLeft(formatCurrency(resumen.baseCaja), 16));
  lines.push(padRight("Total Ingresos:", 27) + padRight("(+)", 5) + padLeft(formatCurrency(resumen.ingresos), 16));
  lines.push(padRight("Total Venta Contado:", 27) + padRight("(+)", 5) + padLeft(formatCurrency(resumen.ventasContado), 16));
  lines.push(padRight("Total Venta Credito:", 27) + padRight("(No)", 5) + padLeft(formatCurrency(resumen.ventasCredito), 16));
  lines.push(padRight("Total Pagados Por Compra:", 27) + padRight("(-)", 5) + padLeft(formatCurrency(resumen.compras), 16));
  lines.push(padRight("Total Egresos:", 27) + padRight("(-)", 5) + padLeft(formatCurrency(resumen.egresos), 16));
  
  lines.push(" ".repeat(15) + "-".repeat(33));
  
  lines.push(padRight("Total En Caja Efectivo:", 30) + padLeft(formatCurrency(resumen.cajaEfectivo), 18));
  lines.push(padRight("Total En Caja:", 30) + padLeft(formatCurrency(resumen.cajaTotal), 18));

  // Unir todo en texto plano Courier monoespaciado
  const contentText = lines.join('\n');

  // Definición del documento pdfmake para ticket de 80mm (226.77 pt)
  const docDefinition = {
    pageSize: {
      width: 250,
      height: 'auto' as any // auto height handles any list size nicely
    },
    pageMargins: [12, 15, 12, 15] as [number, number, number, number],
    content: [
      {
        text: contentText,
        font: 'Courier',
        fontSize: 7.5,
        lineHeight: 1.25
      }
    ],
    defaultStyle: {
      font: 'Courier'
    }
  };

  // Open native save dialog
  const defaultFilename = `ticket_informe_${startDate.replace(/-/g, '')}.pdf`;
  const selectedPath = await save({
    title: 'Guardar Ticket de Informe',
    defaultPath: defaultFilename,
    filters: [
      {
        name: 'Documento PDF',
        extensions: ['pdf']
      }
    ]
  });

  if (!selectedPath) {
    // User cancelled
    return false;
  }

  // Get buffer from pdfmake and invoke rust command to write it
  const buffer = await pdfMake.createPdf(docDefinition).getBuffer();
  const uint8 = new Uint8Array(buffer);
  const bytes = Array.from(uint8);
  await invoke('save_pdf_file', { path: selectedPath, content: bytes });
  return true;
}
