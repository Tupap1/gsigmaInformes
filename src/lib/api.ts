import { invoke } from '@tauri-apps/api/core';

export interface Proveedor {
  id: string;
  numDoc: string;
  tipoDoc: string;
  email: string | null;
  contacto: string | null;
  status: string;
  pais: string | null;
  nombre: string;
  apellido: string | null;
  telefono1: string | null;
  telefono2: string | null;
  direccion1: string | null;
  ciudad: string | null;
  departamento: string | null;
}

export interface CreateProveedorInput {
  numDoc: string;
  tipoDoc: string;
  nombre: string;
  apellido?: string | null;
  telefono1?: string | null;
  telefono2?: string | null;
  email?: string | null;
  contacto?: string | null;
  direccion1?: string | null;
  ciudad?: string | null;
  departamento?: string | null;
}

export interface UpdateProveedorInput {
  nombre: string;
  apellido?: string | null;
  telefono1?: string | null;
  telefono2?: string | null;
  email?: string | null;
  contacto?: string | null;
  direccion1?: string | null;
  ciudad?: string | null;
  departamento?: string | null;
  status?: string | null;
}

export interface DeleteResult {
  success: boolean;
  action: string;
  reason: string;
  message: string;
}

export interface ConnectionStatus {
  read: boolean;
  write: boolean;
}

/**
 * API Wrapper for Tauri commands
 */
export const api = {
  async testConnection(): Promise<ConnectionStatus> {
    return invoke<ConnectionStatus>('test_connection');
  },

  async listProveedores(includeInactive = false): Promise<Proveedor[]> {
    return invoke<Proveedor[]>('list_proveedores', { includeInactive });
  },

  async getProveedor(id: string): Promise<Proveedor> {
    return invoke<Proveedor>('get_proveedor', { id });
  },

  async createProveedor(input: CreateProveedorInput): Promise<string> {
    return invoke<string>('create_proveedor', { input });
  },

  async updateProveedor(id: string, input: UpdateProveedorInput): Promise<void> {
    return invoke<void>('update_proveedor', { id, input });
  },

  async deleteProveedor(id: string): Promise<DeleteResult> {
    return invoke<DeleteResult>('delete_proveedor', { id });
  }
};
