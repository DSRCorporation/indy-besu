import { expect } from 'chai'
import { createBaseDidDocument } from '../../utils/entity-factories'
import { deployIndyDidRegistry, TestableIndyDidRegistry } from '../utils/contract-helpers'
import { IndyDidValidator } from '../utils/contract-helpers'
import { DidError } from '../utils/errors'
import { TestAccounts } from '../utils/test-entities'

describe('DIDContract', function () {
  let didRegistry: TestableIndyDidRegistry
  let didValidator: IndyDidValidator
  let testAccounts: TestAccounts

  beforeEach(async function () {
    const {
      indyDidRegistry: didRegistryInit,
      indyDidValidator: didValidatorInit,
      testAccounts: testAccountsInit,
    } = await deployIndyDidRegistry()

    didRegistry = didRegistryInit
    didValidator = didValidatorInit
    testAccounts = testAccountsInit

    didRegistry.connect(testAccounts.trustee.account)
  })

  describe('Create DID', function () {
    it('Should create and resolve DID document', async function () {
      const did: string = 'did:indy2:testnet:SEp33q43PsdP7nDATyySSH'
      const didDocument = createBaseDidDocument(did)

      await didRegistry.createDid(testAccounts.noRole.account.address, did, didDocument)

      const { document } = await didRegistry.resolveDid(did)

      expect(document).to.be.deep.equal(didDocument)
    })

    it('Should fail if resolving DID does not exist', async function () {
      const did: string = 'did:indy2:testnet:SEp33q43PsdP7nDATyySSH'

      await expect(didRegistry.resolveDid(did))
        .to.revertedWithCustomError(didRegistry.baseInstance, DidError.DidNotFound)
        .withArgs(did)
    })

    it('Should fail if the DID being created already exists', async function () {
      const did: string = 'did:indy2:testnet:SEp33q43PsdP7nDATyySSH'
      const didDocument = createBaseDidDocument(did)

      await didRegistry.createDid(testAccounts.noRole.account.address, did, didDocument)

      await expect(didRegistry.createDid(testAccounts.noRole2.account.address, did, didDocument))
        .to.be.revertedWithCustomError(didRegistry.baseInstance, DidError.DidAlreadyExist)
        .withArgs(did)
    })

    it('Should fail if an incorrect schema is provided for the DID', async function () {
      const did: string = 'indy:indy2:testnet:SEp33q43PsdP7nDATyySSH'
      const didDocument = createBaseDidDocument(did)

      await expect(didRegistry.createDid(testAccounts.noRole.account.address, did, didDocument))
        .to.be.revertedWithCustomError(didValidator.baseInstance, DidError.IncorrectDid)
        .withArgs(did)
    })

    it('Should fail if an unsupported DID method is provided', async function () {
      const did: string = 'did:indy3:testnet:SEp33q43PsdP7nDATyySSH'
      const didDocument = createBaseDidDocument(did)

      await expect(didRegistry.createDid(testAccounts.noRole.account.address, did, didDocument))
        .to.be.revertedWithCustomError(didValidator.baseInstance, DidError.IncorrectDid)
        .withArgs(did)
    })

    it('Should fail if an incorrect DID method-specific-id is provided', async function () {
      const did: string = 'did:indy3:testnet:123456789'
      const didDocument = createBaseDidDocument(did)

      await expect(didRegistry.createDid(testAccounts.noRole.account.address, did, didDocument))
        .revertedWithCustomError(didValidator.baseInstance, DidError.IncorrectDid)
        .withArgs(did)
    })
  })

  describe('Update DID', function () {
    it('Should update DID document', async function () {
      const did: string = 'did:indy2:testnet:SEp33q43PsdP7nDATyySSH'
      const didDocument = createBaseDidDocument(did)

      await didRegistry.createDid(testAccounts.noRole.account.address, did, didDocument)

      await didRegistry.updateDid(did, didDocument)

      const { document } = await didRegistry.resolveDid(did)

      expect(document).to.be.deep.equal(didDocument)
    })

    it('Should fail if the DID creator is not an update txn sender', async function () {
      const did: string = 'did:indy2:testnet:SEp33q43PsdP7nDATyySSH'
      const didDocument = createBaseDidDocument(did)

      await didRegistry.createDid(testAccounts.noRole.account.address, did, didDocument)

      didRegistry.connect(testAccounts.trustee2.account)
      await expect(didRegistry.updateDid(did, didDocument)).to.revertedWithCustomError(
        didRegistry.baseInstance,
        DidError.UnauthorizedSender,
      )
    })

    it('Should fail if the DID being updated does not exists', async function () {
      const did: string = 'did:indy2:testnet:SEp33q43PsdP7nDATyySSH'
      const didDocument = createBaseDidDocument(did)

      await expect(didRegistry.updateDid(did, didDocument))
        .to.revertedWithCustomError(didRegistry.baseInstance, DidError.DidNotFound)
        .withArgs(did)
    })

    it('Should fail if the DID being updated is deactivated', async function () {
      const did: string = 'did:indy2:testnet:SEp33q43PsdP7nDATyySSH'
      const didDocument = createBaseDidDocument(did)

      await didRegistry.createDid(testAccounts.noRole.account.address, did, didDocument)
      await didRegistry.deactivateDid(did)

      await expect(didRegistry.updateDid(did, didDocument))
        .to.revertedWithCustomError(didRegistry.baseInstance, DidError.DidHasBeenDeactivated)
        .withArgs(did)
    })
  })

  describe('Deactivate DID', function () {
    it('Should deactivate DID document', async function () {
      const did: string = 'did:indy2:testnet:SEp33q43PsdP7nDATyySSH'
      const didDocument = createBaseDidDocument(did)

      await didRegistry.createDid(testAccounts.noRole.account.address, did, didDocument)
      await didRegistry.deactivateDid(did)

      const didStorage = await didRegistry.resolveDid(did)

      expect(didStorage.metadata.deactivated).is.true
    })

    it('Should fail if the DID has already been deactivated', async function () {
      const did: string = 'did:indy2:testnet:SEp33q43PsdP7nDATyySSH'
      const didDocument = createBaseDidDocument(did)

      await didRegistry.createDid(testAccounts.noRole.account.address, did, didDocument)
      await didRegistry.deactivateDid(did)

      await expect(didRegistry.deactivateDid(did))
        .to.revertedWithCustomError(didRegistry.baseInstance, DidError.DidHasBeenDeactivated)
        .withArgs(did)
    })

    it('Should fail if the DID being deactivated does not exists', async function () {
      const did: string = 'did:indy2:testnet:SEp33q43PsdP7nDATyySSH'

      await expect(didRegistry.deactivateDid(did))
        .to.revertedWithCustomError(didRegistry.baseInstance, DidError.DidNotFound)
        .withArgs(did)
    })

    it('Should fail if the DID creator is not an deactivate txn sender', async function () {
      const did: string = 'did:indy2:testnet:SEp33q43PsdP7nDATyySSH'
      const didDocument = createBaseDidDocument(did)

      await didRegistry.createDid(testAccounts.noRole.account.address, did, didDocument)

      didRegistry.connect(testAccounts.trustee2.account)
      await expect(didRegistry.deactivateDid(did)).to.revertedWithCustomError(
        didRegistry.baseInstance,
        DidError.UnauthorizedSender,
      )
    })
  })
})
