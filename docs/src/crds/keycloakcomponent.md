# KeycloakComponent

## v1

resource to define a Component within a [KeycloakRealm](./keycloakrealm.md)

|Property|Type|Required|
|:-------|:---|:------:|
|[spec](#spec)|object|✅|
|[spec.definition](#specdefinition)|object|✅|
|[spec.definition.config](#specdefinitionconfig)|object||
|[spec.definition.id](#specdefinitionid)|string||
|[spec.definition.name](#specdefinitionname)|string||
|[spec.definition.parentId](#specdefinitionparentid)|string||
|[spec.definition.providerId](#specdefinitionproviderid)|string||
|[spec.definition.providerType](#specdefinitionprovidertype)|string||
|[spec.definition.subType](#specdefinitionsubtype)|string||
|[spec.options](#specoptions)|object||
|[spec.realmRef](#specrealmref)|string|✅|
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

the KeycloakComponent resource

---

### spec.definition

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[config](#specdefinitionconfig)|object||
|[id](#specdefinitionid)|string||
|[name](#specdefinitionname)|string||
|[parentId](#specdefinitionparentid)|string||
|[providerId](#specdefinitionproviderid)|string||
|[providerType](#specdefinitionprovidertype)|string||
|[subType](#specdefinitionsubtype)|string||

&nbsp;

|Validation Rule|Error Message|
|:--------------|:------------|
|has(self.id) == has(oldSelf.id)|Value is immutable|

*missing*

---

### spec.definition.config

Type: object

*missing*

---

### spec.definition.id

Type: string

|Validation Rule|Error Message|
|:--------------|:------------|
|self == oldSelf|Value is immutable|

*missing*

---

### spec.definition.name

Type: string

*missing*

---

### spec.definition.parentId

Type: string

*missing*

---

### spec.definition.providerId

Type: string

*missing*

---

### spec.definition.providerType

Type: string

*missing*

---

### spec.definition.subType

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