# 📊 Análisis de Base de Datos — Recicladora Boyacá
## Aplicativo de Informes y Gestión de Proveedores

---

## 1. Panorama General

La base de datos proviene de un **POS (Punto de Venta)** con software tipo **GsigmaPOS/Itheke** corriendo sobre **MySQL 5.5** en local. Contiene **4 esquemas relevantes**:

| Esquema | Propósito | Tablas Clave |
|---------|-----------|--------------|
| `adm` | Administración de terceros (clientes, proveedores, personas) | `trc`, `trcadi`, `trcest`, `cliente`, `proveedo` |
| `pv` | Punto de Venta (artículos, compras, ventas, inventario) | `pas`, `artic`, `compra`, `dcmpr`, `venta`, `dvent`, `grupo`, `karde` |
| `sis` | Sistema (usuarios, permisos, parámetros) | `usuario`, `param`, `emp` |
| `tcc` | Tesorería (cuentas por cobrar/pagar) | `cxc`, `cxp`, `nota`, `reccaj` |

---

## 2. Estructura de Tablas Críticas

### 2.1 `adm.trc` — Tabla Maestra de Terceros (Personas/Empresas)

Esta es la **tabla central** que registra toda persona o empresa que interactúa con el negocio.

```sql
CREATE TABLE `trc` (
  `EMPID`       varchar(20)  -- ID empresa
  `TRCID`       varchar(20)  -- ⭐ ID del tercero (PK lógico)
  `TRCNOM`      varchar(200) -- Nombre / Razón Social
  `TRCAPE`      varchar(50)  -- Apellido
  `TRCTEL1`     varchar(20)  -- Teléfono 1
  `TRCTEL2`     varchar(20)  -- Teléfono 2
  `trcema1`     varchar(80)  -- Email 1
  `TRCTIPDOC`   varchar(1)   -- Tipo documento: C=Cédula, N=NIT, E=Extranjería
  `TRCNUMDOC`   varchar(20)  -- Número de documento
  `TRCDIR1`     varchar(80)  -- Dirección 1
  `TRCCIU`      varchar(50)  -- Ciudad
  `TRCPAI`      varchar(50)  -- País
  `TRCNAT`      varchar(1)   -- Naturaleza: J=Jurídica, N=Natural
  `TRCULTMOD`   date         -- Última modificación
  -- ... otros campos
);
```

### 2.2 `pv.proveedo` — Proveedores

Tabla específica de proveedores, vinculada a `trc` mediante `PROCOD ↔ TRCID`.

```sql
CREATE TABLE `proveedo` (
  `PROCOD`      varchar(15)  -- ⭐ Código proveedor (= TRCID en adm.trc)
  `PROCON`      varchar(40)  -- Persona de contacto
  `PRONUMDOC`   varchar(15)  -- Número documento
  `PROTIPDOC`   varchar(1)   -- Tipo documento
  `PROEMA`      varchar(50)  -- Email proveedor
  `status`      varchar(1)   -- Estado: A=Activo, I=Inactivo
  `pais`        varchar(5)   -- País (default: CO)
  -- ... otros campos
);
```

### 2.3 `pv.pas` — Catálogo de Productos/Servicios (PAS)

```sql
CREATE TABLE `pas` (
  `PAS`         varchar(15)  -- ⭐ Código del producto (001, 002, etc.)
  `PASNOM`      varchar(80)  -- Nombre (CHATARRA, ALUMINIO, COBRE, etc.)
  `PASTIPO`     varchar(1)   -- Tipo: A=Artículo, S=Servicio
  `GRUPCODI`    varchar(20)  -- Código del grupo
  `PASCOMP`     bit(1)       -- ¿Se puede comprar?
  `PASFACT`     bit(1)       -- ¿Se puede vender?
  PRIMARY KEY (`PAS`)
);
```

---

## 3. Análisis del Informe: "COMPRAS ACUMULADAS"

El informe tiene estas secciones:

#### Sección 1: Tabla de Compras por Material
- **PAS**: `pas.PAS`
- **Nombre material**: `pas.PASNOM`
- **Cantidad**: Suma de `dcmpr.DCMCAN` agrupado por artículo.
- **Total**: Suma de `(dcmpr.DCMCOS * dcmpr.DCMCAN)` agrupado por artículo.
- **Costo Promedio**: `Total / Cantidad`

#### Sección 2: Resumen de Caja del Período
- **Total Base Caja** (+): `pv.bas_caj`
- **Total Ingresos** (+): `pv.ingcaj`
- **Total Venta Contado** (+): `pv.venta`
- **Total Venta Crédito** (No): `pv.venta`
- **Total Pagados Por Compra** (-): `pv.compra`
- **Total Egresos** (-): `pv.egrcajp`
- **Total En Caja Efectivo**: Base + Ingresos + Contado - Compras - Egresos
- **Total En Caja**: Caja Efectivo + Crédito

---

## 4. Gestión de Proveedores (Seguridad en Escritura)

### Lógica de Inserción
Toda creación de proveedor debe registrarse en transacción en ambas tablas:

1. **`adm.trc`**: Registro genérico de tercero.
2. **`pv.proveedo`**: Registro específico de proveedor.

### Borrado Seguro
Para evitar inconsistencias en el POS histórico:
- **Proveedores sin transacciones**: Si un proveedor fue creado por error y **no tiene compras asociadas** (`pv.compra`), se le puede aplicar un borrado seguro (eliminar de `pv.proveedo` y `adm.trc`).
- **Proveedores con transacciones**: No se pueden borrar físicamente. Se desactivan mediante soft-delete cambiando el campo `status` a `'I'` (Inactivo).
