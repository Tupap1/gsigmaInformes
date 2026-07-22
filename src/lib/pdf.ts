import pdfMake from 'pdfmake/build/pdfmake';
import * as pdfFonts from 'pdfmake/build/vfs_fonts';
import { save } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';
import type { CompraAcumulada, ResumenCaja } from './api';

// Set standard virtual file system which guarantees Roboto font
(pdfMake as any).vfs = (pdfFonts as any).pdfMake?.vfs || (pdfFonts as any).vfs || (pdfMake as any).vfs;

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
  resumen?: ResumenCaja | null
) {
  // 1. Preparar la tabla de compras acumuladas
  const tableBody: any[] = [];
  
  // Encabezados
  tableBody.push([
    { text: 'PAS', style: 'tableHeader' },
    { text: 'MATERIAL', style: 'tableHeader' },
    { text: 'CANTIDAD', style: 'tableHeaderRight' },
    { text: 'TOTAL', style: 'tableHeaderRight' },
    { text: 'CPP', style: 'tableHeaderRight' }
  ]);

  let totalCantidad = 0;
  let totalDinero = 0;

  for (const item of compras) {
    totalCantidad += item.cantidad;
    totalDinero += item.total;
    tableBody.push([
      { text: item.pas, style: 'tableCellCode' },
      { text: item.nombre.toUpperCase(), style: 'tableCell' },
      { text: formatNumber(item.cantidad), style: 'tableCellRight' },
      { text: `$${formatCurrency(item.total)}`, style: 'tableCellRight' },
      { text: `$${formatCurrency(item.costoPromedio)}`, style: 'tableCellRight' }
    ]);
  }

  // Fila de totales
  tableBody.push([
    { text: 'TOTALES', style: 'tableTotalCell', colSpan: 2 },
    {},
    { text: formatNumber(totalCantidad), style: 'tableTotalCellRight' },
    { text: `$${formatCurrency(totalDinero)}`, style: 'tableTotalCellRight' },
    {}
  ]);

  // Date label formatting
  const dateStr = startDate === endDate ? startDate : `${startDate} a ${endDate}`;

  // Document definition
  const docDefinition = {
    pageSize: {
      width: 240, // 80mm roll printer size
      height: 'auto' as any
    },
    pageMargins: [12, 16, 12, 16] as [number, number, number, number],
    content: [
      // Cabecera del ticket
      { text: 'RECICLADORA BOYACÁ', style: 'ticketHeader' },
      { text: 'INFORME DE COMPRAS ACUMULADAS', style: 'ticketSubheader' },
      { text: `Período: ${dateStr}`, style: 'ticketDate' },
      
      { text: 'RESUMEN DE MATERIALES', style: 'sectionHeader' },
      
      // Tabla de compras
      {
        style: 'comprasTable',
        table: {
          headerRows: 1,
          widths: [20, '*', 46, 52, 34],
          body: tableBody
        },
        layout: {
          hLineWidth: (i: number, node: any) => (i === 0 || i === 1 || i === node.table.body.length) ? 1 : 0.5,
          vLineWidth: () => 0,
          hLineColor: () => '#e5e7eb'
        }
      }
    ],
    
    // Estilos del Reporte
    styles: {
      ticketHeader: {
        fontSize: 14,
        bold: true,
        alignment: 'center',
        color: '#111827',
        margin: [0, 0, 0, 2]
      },
      ticketSubheader: {
        fontSize: 10,
        bold: true,
        alignment: 'center',
        color: '#4b5563',
        margin: [0, 0, 0, 2]
      },
      ticketDate: {
        fontSize: 9,
        alignment: 'center',
        color: '#6b7280',
        margin: [0, 0, 0, 12]
      },
      sectionHeader: {
        fontSize: 9.5,
        bold: true,
        color: '#374151',
        margin: [0, 8, 0, 4]
      },
      comprasTable: {
        margin: [0, 2, 0, 8]
      },
      tableHeader: {
        fontSize: 7.5,
        bold: true,
        color: '#374151',
        margin: [0, 2, 0, 2]
      },
      tableHeaderRight: {
        fontSize: 7.5,
        bold: true,
        color: '#374151',
        alignment: 'right',
        margin: [0, 2, 0, 2]
      },
      tableCell: {
        fontSize: 8,
        color: '#111827',
        margin: [0, 3, 0, 3]
      },
      tableCellCode: {
        fontSize: 8,
        color: '#4b5563',
        margin: [0, 3, 0, 3]
      },
      tableCellRight: {
        fontSize: 8,
        color: '#111827',
        alignment: 'right',
        margin: [0, 3, 0, 3]
      },
      tableTotalCell: {
        fontSize: 9,
        bold: true,
        color: '#111827',
        margin: [0, 4, 0, 4]
      },
      tableTotalCellRight: {
        fontSize: 9,
        bold: true,
        color: '#111827',
        alignment: 'right',
        margin: [0, 4, 0, 4]
      },
      resumenTable: {
        margin: [0, 2, 0, 4]
      },
      resumenCell: {
        fontSize: 9,
        color: '#374151',
        margin: [0, 2, 0, 2]
      },
      resumenCellMuted: {
        fontSize: 9,
        color: '#9ca3af',
        margin: [0, 2, 0, 2]
      },
      resumenCellOp: {
        fontSize: 9,
        color: '#6b7280',
        alignment: 'center',
        margin: [0, 2, 0, 2]
      },
      resumenCellOpMuted: {
        fontSize: 9,
        color: '#d1d5db',
        alignment: 'center',
        margin: [0, 2, 0, 2]
      },
      resumenCellVal: {
        fontSize: 9,
        bold: true,
        color: '#111827',
        alignment: 'right',
        margin: [0, 2, 0, 2]
      },
      resumenCellValMuted: {
        fontSize: 9,
        color: '#9ca3af',
        alignment: 'right',
        margin: [0, 2, 0, 2]
      },
      balanceBoxGreen: {
        background: '#ecfdf5',
        padding: [8, 6, 8, 6]
      },
      balanceBoxBlue: {
        background: '#eff6ff',
        padding: [8, 6, 8, 6]
      },
      balanceLabel: {
        fontSize: 10.5,
        bold: true,
        color: '#1f2937'
      },
      balanceValueGreen: {
        fontSize: 12,
        bold: true,
        color: '#059669',
        alignment: 'right'
      },
      balanceValueBlue: {
        fontSize: 12,
        bold: true,
        color: '#2563eb',
        alignment: 'right'
      }
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
  const buffer = await pdfMake.createPdf(docDefinition as any).getBuffer();
  const uint8 = new Uint8Array(buffer);
  const bytes = Array.from(uint8);
  await invoke('save_pdf_file', { path: selectedPath, content: bytes });
  return true;
}
