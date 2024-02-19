import { padLeft } from 'web3-utils'
import { ContractConfig } from '../contractConfig'
import { buildProxySection, slots } from '../helpers'

export interface EthereumDidRegistryConfig extends ContractConfig {
  data: {
    upgradeControlAddress: string
  }
}

export function ethereumDidRegistry(config: EthereumDidRegistryConfig) {
  const { name, address, description } = config
  const storage: any = {}

  storage[slots['0']] = padLeft(config.data.upgradeControlAddress, 64)

  return buildProxySection(name, address, description, storage)
}
