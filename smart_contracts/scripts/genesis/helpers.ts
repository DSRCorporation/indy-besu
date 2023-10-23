import * as fs from 'fs-extra'
import path from 'path'
import { padLeft, sha3 } from 'web3-utils'
import { compiledContractsFolder } from './config'

// tslint:disable-next-line: no-var-requires
const linker = require('solc/linker')

export const slots = {
  '0': '0000000000000000000000000000000000000000000000000000000000000000',
  '1': '0000000000000000000000000000000000000000000000000000000000000001',
  '2': '0000000000000000000000000000000000000000000000000000000000000002',
  '3': '0000000000000000000000000000000000000000000000000000000000000003',
}

const proxyBytecode = readContractBytecode('ERC1967Proxy')
const proxyImplmentationSlot = '360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc'

export function computeContractAddress(name: string) {
  const bytecode = readContractBytecode(name)

  const bytecodeHash = sha3(bytecode)

  sha3(bytecode)?.substring(44)
}

export function buildProxySection(
  name: string,
  address: string,
  comment: string,
  storage: Record<string, string>,
  libraries?: { [libraryName: string]: string },
) {
  let implementationBytecode = readContractBytecode(name)

  if (libraries) {
    implementationBytecode = linker.linkBytecode(implementationBytecode, libraries).split('\n')[0]
  }

  const implementationAddress = sha3(implementationBytecode)!.substring(26)

  storage[proxyImplmentationSlot] = `0x${padLeft(implementationAddress, 64)}`

  return {
    [address]: {
      comment: `Proxy: ${comment}`,
      balance: '0',
      code: `0x${proxyBytecode}`,
      storage,
    },
    [`0x${implementationAddress}`]: {
      comment: `Implementation: ${comment}`,
      balance: '0',
      code: `0x${implementationBytecode}`,
    },
  }
}

export function buildSection(
  name: string,
  address: string,
  comment: string,
  storage: Record<string, string>,
  libraries?: { [libraryName: string]: string },
) {
  let bytecode = readContractBytecode(name)

  if (libraries) {
    bytecode = linker.linkBytecode(bytecode, libraries).split('\n')[0]
  }

  return {
    [address]: {
      comment,
      balance: '0',
      code: `0x${bytecode}`,
      storage,
    },
  }
}

export function readContractBytecode(contractName: string) {
  return fs.readFileSync(
    path.resolve(__dirname, '../../', compiledContractsFolder, `${contractName}.bin-runtime`),
    'utf8',
  )
}
