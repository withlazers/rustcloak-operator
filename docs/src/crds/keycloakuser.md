# KeycloakUser

## v1

resource to define a User within a [KeyclaokRealm](./keycloakrealm.md)

|Property|Type|Required|
|:-------|:---|:------:|
|[spec](#spec)|object|✅|
|[spec.definition](#specdefinition)|object|✅|
|[spec.definition.access](#specdefinitionaccess)|object||
|[spec.definition.applicationRoles](#specdefinitionapplicationroles)|object||
|[spec.definition.attributes](#specdefinitionattributes)|object||
|[spec.definition.clientConsents[]](#specdefinitionclientconsents)|object||
|[spec.definition.clientConsents[].clientId](#specdefinitionclientconsentsclientid)|string||
|[spec.definition.clientConsents[].createdDate](#specdefinitionclientconsentscreateddate)|integer||
|[spec.definition.clientConsents[].grantedClientScopes[]](#specdefinitionclientconsentsgrantedclientscopes)|string||
|[spec.definition.clientConsents[].grantedRealmRoles[]](#specdefinitionclientconsentsgrantedrealmroles)|string||
|[spec.definition.clientConsents[].lastUpdatedDate](#specdefinitionclientconsentslastupdateddate)|integer||
|[spec.definition.clientRoles](#specdefinitionclientroles)|object||
|[spec.definition.createdTimestamp](#specdefinitioncreatedtimestamp)|integer||
|[spec.definition.credentials[]](#specdefinitioncredentials)|object||
|[spec.definition.credentials[].algorithm](#specdefinitioncredentialsalgorithm)|string||
|[spec.definition.credentials[].config](#specdefinitioncredentialsconfig)|object||
|[spec.definition.credentials[].counter](#specdefinitioncredentialscounter)|integer||
|[spec.definition.credentials[].createdDate](#specdefinitioncredentialscreateddate)|integer||
|[spec.definition.credentials[].credentialData](#specdefinitioncredentialscredentialdata)|string||
|[spec.definition.credentials[].device](#specdefinitioncredentialsdevice)|string||
|[spec.definition.credentials[].digits](#specdefinitioncredentialsdigits)|integer||
|[spec.definition.credentials[].hashIterations](#specdefinitioncredentialshashiterations)|integer||
|[spec.definition.credentials[].hashedSaltedValue](#specdefinitioncredentialshashedsaltedvalue)|string||
|[spec.definition.credentials[].id](#specdefinitioncredentialsid)|string||
|[spec.definition.credentials[].period](#specdefinitioncredentialsperiod)|integer||
|[spec.definition.credentials[].priority](#specdefinitioncredentialspriority)|integer||
|[spec.definition.credentials[].salt](#specdefinitioncredentialssalt)|string||
|[spec.definition.credentials[].secretData](#specdefinitioncredentialssecretdata)|string||
|[spec.definition.credentials[].temporary](#specdefinitioncredentialstemporary)|boolean||
|[spec.definition.credentials[].type](#specdefinitioncredentialstype)|string||
|[spec.definition.credentials[].userLabel](#specdefinitioncredentialsuserlabel)|string||
|[spec.definition.credentials[].value](#specdefinitioncredentialsvalue)|string||
|[spec.definition.disableableCredentialTypes[]](#specdefinitiondisableablecredentialtypes)|string||
|[spec.definition.email](#specdefinitionemail)|string||
|[spec.definition.emailVerified](#specdefinitionemailverified)|boolean||
|[spec.definition.enabled](#specdefinitionenabled)|boolean||
|[spec.definition.federatedIdentities[]](#specdefinitionfederatedidentities)|object||
|[spec.definition.federatedIdentities[].identityProvider](#specdefinitionfederatedidentitiesidentityprovider)|string||
|[spec.definition.federatedIdentities[].userId](#specdefinitionfederatedidentitiesuserid)|string||
|[spec.definition.federatedIdentities[].userName](#specdefinitionfederatedidentitiesusername)|string||
|[spec.definition.federationLink](#specdefinitionfederationlink)|string||
|[spec.definition.firstName](#specdefinitionfirstname)|string||
|[spec.definition.groups[]](#specdefinitiongroups)|string||
|[spec.definition.id](#specdefinitionid)|string||
|[spec.definition.lastName](#specdefinitionlastname)|string||
|[spec.definition.notBefore](#specdefinitionnotbefore)|integer||
|[spec.definition.origin](#specdefinitionorigin)|string||
|[spec.definition.realmRoles[]](#specdefinitionrealmroles)|string||
|[spec.definition.requiredActions[]](#specdefinitionrequiredactions)|string||
|[spec.definition.self](#specdefinitionself)|string||
|[spec.definition.serviceAccountClientId](#specdefinitionserviceaccountclientid)|string||
|[spec.definition.socialLinks[]](#specdefinitionsociallinks)|object||
|[spec.definition.socialLinks[].socialProvider](#specdefinitionsociallinkssocialprovider)|string||
|[spec.definition.socialLinks[].socialUserId](#specdefinitionsociallinkssocialuserid)|string||
|[spec.definition.socialLinks[].socialUsername](#specdefinitionsociallinkssocialusername)|string||
|[spec.definition.totp](#specdefinitiontotp)|boolean||
|[spec.definition.userProfileMetadata](#specdefinitionuserprofilemetadata)|object||
|[spec.definition.userProfileMetadata.attributes[]](#specdefinitionuserprofilemetadataattributes)|object||
|[spec.definition.userProfileMetadata.attributes[].annotations](#specdefinitionuserprofilemetadataattributesannotations)|object||
|[spec.definition.userProfileMetadata.attributes[].displayName](#specdefinitionuserprofilemetadataattributesdisplayname)|string||
|[spec.definition.userProfileMetadata.attributes[].group](#specdefinitionuserprofilemetadataattributesgroup)|string||
|[spec.definition.userProfileMetadata.attributes[].multivalued](#specdefinitionuserprofilemetadataattributesmultivalued)|boolean||
|[spec.definition.userProfileMetadata.attributes[].name](#specdefinitionuserprofilemetadataattributesname)|string||
|[spec.definition.userProfileMetadata.attributes[].readOnly](#specdefinitionuserprofilemetadataattributesreadonly)|boolean||
|[spec.definition.userProfileMetadata.attributes[].required](#specdefinitionuserprofilemetadataattributesrequired)|boolean||
|[spec.definition.userProfileMetadata.attributes[].validators](#specdefinitionuserprofilemetadataattributesvalidators)|object||
|[spec.definition.userProfileMetadata.groups[]](#specdefinitionuserprofilemetadatagroups)|object||
|[spec.definition.userProfileMetadata.groups[].annotations](#specdefinitionuserprofilemetadatagroupsannotations)|object||
|[spec.definition.userProfileMetadata.groups[].displayDescription](#specdefinitionuserprofilemetadatagroupsdisplaydescription)|string||
|[spec.definition.userProfileMetadata.groups[].displayHeader](#specdefinitionuserprofilemetadatagroupsdisplayheader)|string||
|[spec.definition.userProfileMetadata.groups[].name](#specdefinitionuserprofilemetadatagroupsname)|string||
|[spec.definition.username](#specdefinitionusername)|string||
|[spec.options](#specoptions)|object||
|[spec.realmRef](#specrealmref)|string|✅|
|[spec.userSecret](#specusersecret)|object||
|[spec.userSecret.passwordKey](#specusersecretpasswordkey)|string||
|[spec.userSecret.secretName](#specusersecretsecretname)|string|✅|
|[spec.userSecret.usernameKey](#specusersecretusernamekey)|string||
|[status](#status)|object||
|[status.conditions[]](#statusconditions)|object||
|[status.conditions[].lastTransitionTime](#statusconditionslasttransitiontime)|string||
|[status.conditions[].lastUpdateTime](#statusconditionslastupdatetime)|string||
|[status.conditions[].message](#statusconditionsmessage)|string||
|[status.conditions[].reason](#statusconditionsreason)|string||
|[status.conditions[].status](#statusconditionsstatus)|string|✅|
|[status.conditions[].type](#statusconditionstype)|string|✅|
|[status.message](#statusmessage)|string||
|[status.ready](#statusready)|boolean|✅|
|[status.resourcePath](#statusresourcepath)|string||
|[status.status](#statusstatus)|string||

---

### spec

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[definition](#specdefinition)|object|✅|
|[options](#specoptions)|object||
|[realmRef](#specrealmref)|string|✅|
|[userSecret](#specusersecret)|object||

the KeycloakUser resource

---

### spec.definition

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[access](#specdefinitionaccess)|object||
|[applicationRoles](#specdefinitionapplicationroles)|object||
|[attributes](#specdefinitionattributes)|object||
|[clientConsents[]](#specdefinitionclientconsents)|object||
|[clientRoles](#specdefinitionclientroles)|object||
|[createdTimestamp](#specdefinitioncreatedtimestamp)|integer||
|[credentials[]](#specdefinitioncredentials)|object||
|[disableableCredentialTypes[]](#specdefinitiondisableablecredentialtypes)|string||
|[email](#specdefinitionemail)|string||
|[emailVerified](#specdefinitionemailverified)|boolean||
|[enabled](#specdefinitionenabled)|boolean||
|[federatedIdentities[]](#specdefinitionfederatedidentities)|object||
|[federationLink](#specdefinitionfederationlink)|string||
|[firstName](#specdefinitionfirstname)|string||
|[groups[]](#specdefinitiongroups)|string||
|[id](#specdefinitionid)|string||
|[lastName](#specdefinitionlastname)|string||
|[notBefore](#specdefinitionnotbefore)|integer||
|[origin](#specdefinitionorigin)|string||
|[realmRoles[]](#specdefinitionrealmroles)|string||
|[requiredActions[]](#specdefinitionrequiredactions)|string||
|[self](#specdefinitionself)|string||
|[serviceAccountClientId](#specdefinitionserviceaccountclientid)|string||
|[socialLinks[]](#specdefinitionsociallinks)|object||
|[totp](#specdefinitiontotp)|boolean||
|[userProfileMetadata](#specdefinitionuserprofilemetadata)|object||
|[username](#specdefinitionusername)|string||

&nbsp;

|Validation Rule|Error Message|
|:--------------|:------------|
|has(self.id) == has(oldSelf.id)|Value is immutable|

*missing*

---

### spec.definition.access

Type: object

*missing*

---

### spec.definition.applicationRoles

Type: object

*missing*

---

### spec.definition.attributes

Type: object

*missing*

---

### spec.definition.clientConsents[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[clientId](#specdefinitionclientconsentsclientid)|string||
|[createdDate](#specdefinitionclientconsentscreateddate)|integer||
|[grantedClientScopes[]](#specdefinitionclientconsentsgrantedclientscopes)|string||
|[grantedRealmRoles[]](#specdefinitionclientconsentsgrantedrealmroles)|string||
|[lastUpdatedDate](#specdefinitionclientconsentslastupdateddate)|integer||

*missing*

---

### spec.definition.clientConsents[].clientId

Type: string

*missing*

---

### spec.definition.clientConsents[].createdDate

Type: integer

*missing*

---

### spec.definition.clientConsents[].grantedClientScopes[]

Type: string

*missing*

---

### spec.definition.clientConsents[].grantedRealmRoles[]

Type: string

*missing*

---

### spec.definition.clientConsents[].lastUpdatedDate

Type: integer

*missing*

---

### spec.definition.clientRoles

Type: object

*missing*

---

### spec.definition.createdTimestamp

Type: integer

*missing*

---

### spec.definition.credentials[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[algorithm](#specdefinitioncredentialsalgorithm)|string||
|[config](#specdefinitioncredentialsconfig)|object||
|[counter](#specdefinitioncredentialscounter)|integer||
|[createdDate](#specdefinitioncredentialscreateddate)|integer||
|[credentialData](#specdefinitioncredentialscredentialdata)|string||
|[device](#specdefinitioncredentialsdevice)|string||
|[digits](#specdefinitioncredentialsdigits)|integer||
|[hashIterations](#specdefinitioncredentialshashiterations)|integer||
|[hashedSaltedValue](#specdefinitioncredentialshashedsaltedvalue)|string||
|[id](#specdefinitioncredentialsid)|string||
|[period](#specdefinitioncredentialsperiod)|integer||
|[priority](#specdefinitioncredentialspriority)|integer||
|[salt](#specdefinitioncredentialssalt)|string||
|[secretData](#specdefinitioncredentialssecretdata)|string||
|[temporary](#specdefinitioncredentialstemporary)|boolean||
|[type](#specdefinitioncredentialstype)|string||
|[userLabel](#specdefinitioncredentialsuserlabel)|string||
|[value](#specdefinitioncredentialsvalue)|string||

*missing*

---

### spec.definition.credentials[].algorithm

Type: string

*missing*

---

### spec.definition.credentials[].config

Type: object

*missing*

---

### spec.definition.credentials[].counter

Type: integer

*missing*

---

### spec.definition.credentials[].createdDate

Type: integer

*missing*

---

### spec.definition.credentials[].credentialData

Type: string

*missing*

---

### spec.definition.credentials[].device

Type: string

*missing*

---

### spec.definition.credentials[].digits

Type: integer

*missing*

---

### spec.definition.credentials[].hashIterations

Type: integer

*missing*

---

### spec.definition.credentials[].hashedSaltedValue

Type: string

*missing*

---

### spec.definition.credentials[].id

Type: string

*missing*

---

### spec.definition.credentials[].period

Type: integer

*missing*

---

### spec.definition.credentials[].priority

Type: integer

*missing*

---

### spec.definition.credentials[].salt

Type: string

*missing*

---

### spec.definition.credentials[].secretData

Type: string

*missing*

---

### spec.definition.credentials[].temporary

Type: boolean

*missing*

---

### spec.definition.credentials[].type

Type: string

*missing*

---

### spec.definition.credentials[].userLabel

Type: string

*missing*

---

### spec.definition.credentials[].value

Type: string

*missing*

---

### spec.definition.disableableCredentialTypes[]

Type: string

*missing*

---

### spec.definition.email

Type: string

*missing*

---

### spec.definition.emailVerified

Type: boolean

*missing*

---

### spec.definition.enabled

Type: boolean

*missing*

---

### spec.definition.federatedIdentities[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[identityProvider](#specdefinitionfederatedidentitiesidentityprovider)|string||
|[userId](#specdefinitionfederatedidentitiesuserid)|string||
|[userName](#specdefinitionfederatedidentitiesusername)|string||

*missing*

---

### spec.definition.federatedIdentities[].identityProvider

Type: string

*missing*

---

### spec.definition.federatedIdentities[].userId

Type: string

*missing*

---

### spec.definition.federatedIdentities[].userName

Type: string

*missing*

---

### spec.definition.federationLink

Type: string

*missing*

---

### spec.definition.firstName

Type: string

*missing*

---

### spec.definition.groups[]

Type: string

*missing*

---

### spec.definition.id

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

*missing*

---

### spec.definition.lastName

Type: string

*missing*

---

### spec.definition.notBefore

Type: integer

*missing*

---

### spec.definition.origin

Type: string

*missing*

---

### spec.definition.realmRoles[]

Type: string

*missing*

---

### spec.definition.requiredActions[]

Type: string

*missing*

---

### spec.definition.self

Type: string

*missing*

---

### spec.definition.serviceAccountClientId

Type: string

*missing*

---

### spec.definition.socialLinks[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[socialProvider](#specdefinitionsociallinkssocialprovider)|string||
|[socialUserId](#specdefinitionsociallinkssocialuserid)|string||
|[socialUsername](#specdefinitionsociallinkssocialusername)|string||

*missing*

---

### spec.definition.socialLinks[].socialProvider

Type: string

*missing*

---

### spec.definition.socialLinks[].socialUserId

Type: string

*missing*

---

### spec.definition.socialLinks[].socialUsername

Type: string

*missing*

---

### spec.definition.totp

Type: boolean

*missing*

---

### spec.definition.userProfileMetadata

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[attributes[]](#specdefinitionuserprofilemetadataattributes)|object||
|[groups[]](#specdefinitionuserprofilemetadatagroups)|object||

*missing*

---

### spec.definition.userProfileMetadata.attributes[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[annotations](#specdefinitionuserprofilemetadataattributesannotations)|object||
|[displayName](#specdefinitionuserprofilemetadataattributesdisplayname)|string||
|[group](#specdefinitionuserprofilemetadataattributesgroup)|string||
|[multivalued](#specdefinitionuserprofilemetadataattributesmultivalued)|boolean||
|[name](#specdefinitionuserprofilemetadataattributesname)|string||
|[readOnly](#specdefinitionuserprofilemetadataattributesreadonly)|boolean||
|[required](#specdefinitionuserprofilemetadataattributesrequired)|boolean||
|[validators](#specdefinitionuserprofilemetadataattributesvalidators)|object||

*missing*

---

### spec.definition.userProfileMetadata.attributes[].annotations

Type: object

*missing*

---

### spec.definition.userProfileMetadata.attributes[].displayName

Type: string

*missing*

---

### spec.definition.userProfileMetadata.attributes[].group

Type: string

*missing*

---

### spec.definition.userProfileMetadata.attributes[].multivalued

Type: boolean

*missing*

---

### spec.definition.userProfileMetadata.attributes[].name

Type: string

*missing*

---

### spec.definition.userProfileMetadata.attributes[].readOnly

Type: boolean

*missing*

---

### spec.definition.userProfileMetadata.attributes[].required

Type: boolean

*missing*

---

### spec.definition.userProfileMetadata.attributes[].validators

Type: object

*missing*

---

### spec.definition.userProfileMetadata.groups[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[annotations](#specdefinitionuserprofilemetadatagroupsannotations)|object||
|[displayDescription](#specdefinitionuserprofilemetadatagroupsdisplaydescription)|string||
|[displayHeader](#specdefinitionuserprofilemetadatagroupsdisplayheader)|string||
|[name](#specdefinitionuserprofilemetadatagroupsname)|string||

*missing*

---

### spec.definition.userProfileMetadata.groups[].annotations

Type: object

*missing*

---

### spec.definition.userProfileMetadata.groups[].displayDescription

Type: string

*missing*

---

### spec.definition.userProfileMetadata.groups[].displayHeader

Type: string

*missing*

---

### spec.definition.userProfileMetadata.groups[].name

Type: string

*missing*

---

### spec.definition.username

Type: string

*missing*

---

### spec.options

Type: object

Options for the request to the Keycloak Admin API.

---

### spec.realmRef

Type: string

the name of the kubernetes object that created the realm.

---

### spec.userSecret

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[passwordKey](#specusersecretpasswordkey)|string||
|[secretName](#specusersecretsecretname)|string|✅|
|[usernameKey](#specusersecretusernamekey)|string||

*missing*

---

### spec.userSecret.passwordKey

Type: string

*missing*

---

### spec.userSecret.secretName

Type: string

*missing*

---

### spec.userSecret.usernameKey

Type: string

*missing*

---

### status

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[conditions[]](#statusconditions)|object||
|[message](#statusmessage)|string||
|[ready](#statusready)|boolean|✅|
|[resourcePath](#statusresourcepath)|string||
|[status](#statusstatus)|string||

*missing*

---

### status.conditions[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[lastTransitionTime](#statusconditionslasttransitiontime)|string||
|[lastUpdateTime](#statusconditionslastupdatetime)|string||
|[message](#statusconditionsmessage)|string||
|[reason](#statusconditionsreason)|string||
|[status](#statusconditionsstatus)|string|✅|
|[type](#statusconditionstype)|string|✅|

*missing*

---

### status.conditions[].lastTransitionTime

Type: string

Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON.  Wrappers are provided for many of the factory methods that the time package offers.

---

### status.conditions[].lastUpdateTime

Type: string

Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON.  Wrappers are provided for many of the factory methods that the time package offers.

---

### status.conditions[].message

Type: string

*missing*

---

### status.conditions[].reason

Type: string

*missing*

---

### status.conditions[].status

Type: string

*missing*

---

### status.conditions[].type

Type: string

*missing*

---

### status.message

Type: string

*missing*

---

### status.ready

Type: boolean

*missing*

---

### status.resourcePath

Type: string

*missing*

---

### status.status

Type: string

*missing*