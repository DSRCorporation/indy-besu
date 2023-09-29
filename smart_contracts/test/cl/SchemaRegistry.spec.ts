import { loadFixture } from '@nomicfoundation/hardhat-network-helpers'
import { expect } from 'chai'
import { DidRegistry, SchemaRegistry } from '../../contracts-ts'
import { Contract } from '../../utils'
import { createBaseDidDocument, createFakeSignature, createSchemaData } from '../utils'

class TestableSchemaRegistry extends SchemaRegistry {
  public get baseInstance() {
    return this.instance
  }
}

describe('SchemaRegistry', function () {
  const issuerId = 'did:indy2:mainnet:SEp33q43PsdP7nDATyySSH'

  async function deploySchemaContractFixture() {
    const didValidator = new Contract('DidValidator')
    await didValidator.deploy()

    const didRegistry = new DidRegistry()
    await didRegistry.deploy({ libraries: [didValidator] })

    const schemaValidator = new Contract('SchemaValidator')
    await schemaValidator.deploy()

    const didDocument = createBaseDidDocument(issuerId)
    const signature = createFakeSignature(issuerId)

    await didRegistry.createDid(didDocument, [signature])

    return new TestableSchemaRegistry().deploy({ params: [didRegistry.address], libraries: [schemaValidator] })
  }

  describe('Add/Resolve Schema', function () {
    it('Should create and resolve Schema', async function () {
      const schemaRegistry = await loadFixture(deploySchemaContractFixture)

      const schemaData = createSchemaData({ issuerId: issuerId })

      await schemaRegistry.createSchema(schemaData)
      const resolvedSchema = await schemaRegistry.resolveSchema(schemaData.id)

      expect(resolvedSchema.data).to.be.deep.equal(schemaData)
    })

    it('Should fail if the Schema ID already exists', async function () {
      const schemaRegistry = await loadFixture(deploySchemaContractFixture)
  
      const schemaData = createSchemaData({ issuerId: issuerId })
  
      await schemaRegistry.createSchema(schemaData)
  
      await expect(schemaRegistry.createSchema(schemaData))
        .to.be.revertedWithCustomError(schemaRegistry.baseInstance, 'SchemaIdExist')
        .withArgs(schemaData.id)
    })
  
    it('Should fail if Schema is being created with non-existing Issuer ID', async function () {
      const schemaRegistry = await loadFixture(deploySchemaContractFixture)
  
      const schemaData = createSchemaData({ issuerId: 'did:indy2:mainnet:GEzcdDLhCpGCYRHW82kjHd' })
  
      await expect(schemaRegistry.createSchema(schemaData))
        .to.be.revertedWithCustomError(schemaRegistry.baseInstance, 'IssuerNotFound')
        .withArgs(schemaData.issuerId)
    })
  
    it('Should fail if Schema is being created with empty name', async function () {
      const schemaRegistry = await loadFixture(deploySchemaContractFixture)
  
      const schemaData = createSchemaData({ issuerId: issuerId, name: "" })
  
      await expect(schemaRegistry.createSchema(schemaData))
        .to.be.revertedWithCustomError(schemaRegistry.baseInstance, 'FieldRequired')
        .withArgs('name')
    })
  
    it('Should fail if Schema is being created with empty version', async function () {
      const schemaRegistry = await loadFixture(deploySchemaContractFixture)
  
      const schemaData = createSchemaData({ issuerId: issuerId, version: "" })
  
      await expect(schemaRegistry.createSchema(schemaData))
        .to.be.revertedWithCustomError(schemaRegistry.baseInstance, 'FieldRequired')
        .withArgs('version')
    })
  
    it('Should fail if Schema is being created without attributes', async function () {
      const schemaRegistry = await loadFixture(deploySchemaContractFixture)
  
      const schemaData = createSchemaData({ issuerId: issuerId, attrNames: [] })
  
      await expect(schemaRegistry.createSchema(schemaData))
        .to.be.revertedWithCustomError(schemaRegistry.baseInstance, 'FieldRequired')
        .withArgs('attributes')
    })
  
    it('Should fail if Schema is being created with invalid Schema ID', async function () {
      const schemaRegistry = await loadFixture(deploySchemaContractFixture)
  
      const schemaData = createSchemaData({ issuerId: issuerId })
      schemaData.id = "SEp33q43PsdP7nDATyySSH:2:BasicSchema:1.0.0"
  
      await expect(schemaRegistry.createSchema(schemaData))
        .to.be.revertedWithCustomError(schemaRegistry.baseInstance, 'InvalidSchemId')
        .withArgs(schemaData.id)
    })
  })
})
