-- Crear esquemas principales
CREATE DATABASE IF NOT EXISTS `adm` CHARACTER SET utf8 COLLATE utf8_general_ci;
CREATE DATABASE IF NOT EXISTS `pv` CHARACTER SET utf8 COLLATE utf8_general_ci;
CREATE DATABASE IF NOT EXISTS `sis` CHARACTER SET utf8 COLLATE utf8_general_ci;
CREATE DATABASE IF NOT EXISTS `tcc` CHARACTER SET utf8 COLLATE utf8_general_ci;

-- ======================================================
-- ESQUEMA ADM
-- ======================================================
USE `adm`;

CREATE TABLE IF NOT EXISTS `trc` (
  `EMPID` varchar(20) NOT NULL DEFAULT '000000000000001',
  `TRCID` varchar(20) NOT NULL,
  `TRCNOM` varchar(200) NOT NULL,
  `TRCAPE` varchar(50) DEFAULT '',
  `TRCTEL1` varchar(20) DEFAULT '',
  `TRCTEL2` varchar(20) DEFAULT '',
  `trcema1` varchar(80) DEFAULT '',
  `TRCTIPDOC` varchar(1) NOT NULL,
  `TRCNUMDOC` varchar(20) NOT NULL,
  `TRCDIR1` varchar(80) DEFAULT '',
  `TRCCIU` varchar(50) DEFAULT '',
  `TRCPAI` varchar(50) DEFAULT 'CO',
  `TRCNAT` varchar(1) NOT NULL,
  `TRCDEPA` varchar(50) DEFAULT '',
  `TRCTIP` varchar(20) NOT NULL DEFAULT 'PROVEEDOR',
  `TRCULTMOD` date DEFAULT NULL,
  PRIMARY KEY (`TRCID`),
  KEY `idx_numdoc` (`TRCNUMDOC`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

-- ======================================================
-- ESQUEMA PV
-- ======================================================
USE `pv`;

CREATE TABLE IF NOT EXISTS `proveedo` (
  `PROCOD` varchar(15) NOT NULL,
  `PROCON` varchar(40) DEFAULT '',
  `PRONUMDOC` varchar(15) NOT NULL,
  `PROTIPDOC` varchar(1) NOT NULL,
  `PROEMA` varchar(50) DEFAULT '',
  `EMPID` varchar(20) NOT NULL DEFAULT '000000000000001',
  `status` varchar(1) NOT NULL DEFAULT 'A',
  `pais` varchar(5) DEFAULT 'CO',
  `PROPAGCOM` varchar(5) NOT NULL DEFAULT 'N',
  `PROFECMOD` date DEFAULT NULL,
  `respfisc` varchar(200) NOT NULL DEFAULT 'O-99,',
  `taxscheme` varchar(50) NOT NULL DEFAULT 'ZZ,',
  PRIMARY KEY (`PROCOD`),
  KEY `idx_numdoc` (`PRONUMDOC`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

CREATE TABLE IF NOT EXISTS `pas` (
  `PAS` varchar(15) NOT NULL,
  `PASNOM` varchar(80) NOT NULL,
  `PASTIPO` varchar(1) NOT NULL,
  `GRUPCODI` varchar(20) DEFAULT '',
  `PASCOMP` tinyint(1) NOT NULL DEFAULT '1',
  `PASFACT` tinyint(1) NOT NULL DEFAULT '1',
  PRIMARY KEY (`PAS`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

-- Tablas de compra anualizadas (Año actual, 2024, 2025)
CREATE TABLE IF NOT EXISTS `compra` (
  `EMPID` varchar(20) NOT NULL DEFAULT '000000000000001',
  `COMNUM` varchar(20) NOT NULL,
  `COMPRO` varchar(15) NOT NULL,
  `COMFEC` date NOT NULL,
  `COMEST` varchar(1) NOT NULL DEFAULT 'C',
  `COMTOT` double NOT NULL DEFAULT '0',
  PRIMARY KEY (`COMNUM`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

CREATE TABLE IF NOT EXISTS `compra24` (
  `EMPID` varchar(20) NOT NULL DEFAULT '000000000000001',
  `COMNUM` varchar(20) NOT NULL,
  `COMPRO` varchar(15) NOT NULL,
  `COMFEC` date NOT NULL,
  `COMEST` varchar(1) NOT NULL DEFAULT 'C',
  `COMTOT` double NOT NULL DEFAULT '0',
  PRIMARY KEY (`COMNUM`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

CREATE TABLE IF NOT EXISTS `compra25` (
  `EMPID` varchar(20) NOT NULL DEFAULT '000000000000001',
  `COMNUM` varchar(20) NOT NULL,
  `COMPRO` varchar(15) NOT NULL,
  `COMFEC` date NOT NULL,
  `COMEST` varchar(1) NOT NULL DEFAULT 'C',
  `COMTOT` double NOT NULL DEFAULT '0',
  PRIMARY KEY (`COMNUM`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

-- Detalles de compras
CREATE TABLE IF NOT EXISTS `dcmpr` (
  `DCMNUM` varchar(20) NOT NULL,
  `DCMART` varchar(15) NOT NULL,
  `DCMCAN` decimal(12,4) NOT NULL DEFAULT '0.0000',
  `DCMCOS` decimal(15,2) NOT NULL DEFAULT '0.00'
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

CREATE TABLE IF NOT EXISTS `dcmpr24` (
  `DCMNUM` varchar(20) NOT NULL,
  `DCMART` varchar(15) NOT NULL,
  `DCMCAN` decimal(12,4) NOT NULL DEFAULT '0.0000',
  `DCMCOS` decimal(15,2) NOT NULL DEFAULT '0.00'
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

CREATE TABLE IF NOT EXISTS `dcmpr25` (
  `DCMNUM` varchar(20) NOT NULL,
  `DCMART` varchar(15) NOT NULL,
  `DCMCAN` decimal(12,4) NOT NULL DEFAULT '0.0000',
  `DCMCOS` decimal(15,2) NOT NULL DEFAULT '0.00'
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

-- Tablas de Caja
CREATE TABLE IF NOT EXISTS `bas_caj` (
  `BAS_FECHA` date NOT NULL DEFAULT '2000-01-01',
  `BAS_VALOR` double NOT NULL DEFAULT '0',
  `BASEST` varchar(1) NOT NULL DEFAULT 'C'
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

CREATE TABLE IF NOT EXISTS `ingcaj` (
  `INGFECHA` date NOT NULL DEFAULT '2000-01-01',
  `INGVALOR` double NOT NULL DEFAULT '0',
  `INGESTADO` varchar(3) NOT NULL DEFAULT 'C'
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

CREATE TABLE IF NOT EXISTS `egrcajp` (
  `EGRFECHA` date NOT NULL DEFAULT '2000-01-01',
  `EGRVALOR` double NOT NULL DEFAULT '0',
  `EGRESTADO` varchar(3) NOT NULL DEFAULT 'C'
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

-- Tablas de Venta Anualizadas
CREATE TABLE IF NOT EXISTS `venta` (
  `VENFEC` date NOT NULL DEFAULT '2000-01-01',
  `VENVAL` double NOT NULL DEFAULT '0',
  `VENEST` varchar(1) NOT NULL DEFAULT 'C',
  `TRNTIPO` varchar(5) NOT NULL DEFAULT ''
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

CREATE TABLE IF NOT EXISTS `venta24` (
  `VENFEC` date NOT NULL DEFAULT '2000-01-01',
  `VENVAL` double NOT NULL DEFAULT '0',
  `VENEST` varchar(1) NOT NULL DEFAULT 'C',
  `TRNTIPO` varchar(5) NOT NULL DEFAULT ''
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

CREATE TABLE IF NOT EXISTS `venta25` (
  `VENFEC` date NOT NULL DEFAULT '2000-01-01',
  `VENVAL` double NOT NULL DEFAULT '0',
  `VENEST` varchar(1) NOT NULL DEFAULT 'C',
  `TRNTIPO` varchar(5) NOT NULL DEFAULT ''
) ENGINE=InnoDB DEFAULT CHARSET=utf8;


-- ======================================================
-- INSERCIÓN DE DATOS DE PRUEBA
-- ======================================================

-- 1. Terceros en adm.trc
INSERT INTO `adm`.`trc` (EMPID, TRCID, TRCNOM, TRCAPE, TRCTEL1, TRCTEL2, trcema1, TRCTIPDOC, TRCNUMDOC, TRCDIR1, TRCCIU, TRCPAI, TRCNAT, TRCDEPA, TRCTIP, TRCULTMOD) VALUES
('000000000000001', '900120000000001', 'PROVEEDOR CON COMPRAS S.A.S.', '', '3001112233', '', 'compras@proveedorA.com', 'N', '900123456', 'Calle 10 # 5-20', 'Tunja', 'CO', 'J', 'Boyacá', 'PROVEEDOR', '2026-07-15'),
('000000000000001', '900980000000002', 'PROVEEDOR SIN COMPRAS S.A.S.', '', '3004445566', '', 'sincompras@proveedorB.com', 'N', '900987654', 'Avenida Central # 12-45', 'Duitama', 'CO', 'J', 'Boyacá', 'PROVEEDOR', '2026-07-15'),
('000000000000001', '123450000000003', 'JUAN', 'PEREZ', '3109998877', '', 'juan.perez@email.com', 'C', '12345678', 'Carrera 8 # 22-10', 'Sogamoso', 'CO', 'N', 'Boyacá', 'PROVEEDOR', '2026-07-15');

-- 2. Proveedores en pv.proveedo
INSERT INTO `pv`.`proveedo` (PROCOD, PROCON, PRONUMDOC, PROTIPDOC, PROEMA, EMPID, status, pais, PROFECMOD) VALUES
('900120000000001', 'CARLOS GOMEZ', '900123456', 'N', 'compras@proveedorA.com', '000000000000001', 'A', 'CO', '2026-07-15'),
('900980000000002', 'MARIA RODRIGUEZ', '900987654', 'N', 'sincompras@proveedorB.com', '000000000000001', 'A', 'CO', '2026-07-15'),
('123450000000003', 'JUAN PEREZ', '12345678', 'C', 'juan.perez@email.com', '000000000000001', 'I', 'CO', '2026-07-15');

-- 3. Productos/Materiales en pv.pas
INSERT INTO `pv`.`pas` (PAS, PASNOM, PASTIPO, GRUPCODI, PASCOMP, PASFACT) VALUES
('001', 'COBRE', 'A', 'METALES', 1, 1),
('002', 'ALUMINIO', 'A', 'METALES', 1, 1),
('003', 'CHATARRA', 'A', 'METALES', 1, 1);

-- 4. Compras de prueba para simular historial de transacciones en el proveedor A
-- Compra en año actual (2026)
INSERT INTO `pv`.`compra` (EMPID, COMNUM, COMPRO, COMFEC, COMEST, COMTOT) VALUES
('000000000000001', 'COM-2026-001', '900120000000001', '2026-07-10', 'C', 500000.00);
INSERT INTO `pv`.`dcmpr` (DCMNUM, DCMART, DCMCAN, DCMCOS) VALUES
('COM-2026-001', '001', 10.0000, 50000.00);

-- Compra histórica (2024)
INSERT INTO `pv`.`compra24` (EMPID, COMNUM, COMPRO, COMFEC, COMEST, COMTOT) VALUES
('000000000000001', 'COM-2024-001', '900120000000001', '2024-05-15', 'C', 150000.00);
INSERT INTO `pv`.`dcmpr24` (DCMNUM, DCMART, DCMCAN, DCMCOS) VALUES
('COM-2024-001', '002', 5.0000, 30000.00);

-- 5. Datos de Caja
INSERT INTO `pv`.`bas_caj` (BAS_FECHA, BAS_VALOR, BASEST) VALUES
('2026-07-10', 150000.00, 'C');

INSERT INTO `pv`.`ingcaj` (INGFECHA, INGVALOR, INGESTADO) VALUES
('2026-07-11', 50000.00, 'C');

INSERT INTO `pv`.`egrcajp` (EGRFECHA, EGRVALOR, EGRESTADO) VALUES
('2026-07-12', 20000.00, 'C');

-- 6. Datos de Ventas
-- Ventas año actual (2026)
INSERT INTO `pv`.`venta` (VENFEC, VENVAL, VENEST, TRNTIPO) VALUES
('2026-07-13', 300000.00, 'C', 'CON'),
('2026-07-14', 200000.00, 'C', 'CRE');

-- Ventas históricas (2024)
INSERT INTO `pv`.`venta24` (VENFEC, VENVAL, VENEST, TRNTIPO) VALUES
('2024-05-15', 100000.00, 'C', 'CON');

-- ======================================================
-- CONFIGURACIÓN DE USUARIOS Y PRIVILEGIOS (MySQL 5.5 compatible)
-- ======================================================

-- 1. Usuario Read-Only (reci_read)
GRANT SELECT ON `adm`.* TO 'reci_read'@'%' IDENTIFIED BY 'read_pass_123';
GRANT SELECT ON `pv`.* TO 'reci_read'@'%' IDENTIFIED BY 'read_pass_123';
GRANT SELECT ON `sis`.* TO 'reci_read'@'%' IDENTIFIED BY 'read_pass_123';
GRANT SELECT ON `tcc`.* TO 'reci_read'@'%' IDENTIFIED BY 'read_pass_123';

-- 2. Usuario de Escritura Restringido (reci_write)
GRANT SELECT ON `adm`.* TO 'reci_write'@'%' IDENTIFIED BY 'write_pass_123';
GRANT SELECT ON `pv`.* TO 'reci_write'@'%' IDENTIFIED BY 'write_pass_123';
GRANT SELECT ON `sis`.* TO 'reci_write'@'%' IDENTIFIED BY 'write_pass_123';
GRANT SELECT ON `tcc`.* TO 'reci_write'@'%' IDENTIFIED BY 'write_pass_123';

-- Privilegios de escritura exclusivos en adm.trc y pv.proveedo
GRANT INSERT, UPDATE, DELETE ON `adm`.`trc` TO 'reci_write'@'%';
GRANT INSERT, UPDATE, DELETE ON `pv`.`proveedo` TO 'reci_write'@'%';

FLUSH PRIVILEGES;
