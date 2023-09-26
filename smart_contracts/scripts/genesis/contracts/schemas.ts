import { ContractConfig } from '../contractConfig'
import { config } from '../config'
import { buildSection } from '../helpers'

export interface SchemasConfig extends ContractConfig {
  data: {
    schemas: Array<{ id: string; data: { name: string } }>
  }
}

export function schemas() {
  const { name, address, description } = config.schemas
  const storage: any = {}
  return buildSection(name, address, description, storage)
}