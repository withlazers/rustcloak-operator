# KeycloakClientScope

## v1

resource to define a Scope within a [KeycloakClient](./keycloakclient.md)

|Property|Type|Required|
|:-------|:---|:------:|
|[spec](#spec)|object|✅|
|[spec.definition](#specdefinition)|object|✅|
|[spec.definition.attributes](#specdefinitionattributes)|object||
|[spec.definition.description](#specdefinitiondescription)|string||
|[spec.definition.id](#specdefinitionid)|string||
|[spec.definition.name](#specdefinitionname)|string||
|[spec.definition.protocol](#specdefinitionprotocol)|string||
|[spec.definition.protocolMappers[]](#specdefinitionprotocolmappers)|object||
|[spec.definition.protocolMappers[].config](#specdefinitionprotocolmappersconfig)|object||
|[spec.definition.protocolMappers[].consentRequired](#specdefinitionprotocolmappersconsentrequired)|boolean||
|[spec.definition.protocolMappers[].consentText](#specdefinitionprotocolmappersconsenttext)|string||
|[spec.definition.protocolMappers[].id](#specdefinitionprotocolmappersid)|string||
|[spec.definition.protocolMappers[].name](#specdefinitionprotocolmappersname)|string||
|[spec.definition.protocolMappers[].protocol](#specdefinitionprotocolmappersprotocol)|string||
|[spec.definition.protocolMappers[].protocolMapper](#specdefinitionprotocolmappersprotocolmapper)|string||
|[spec.isTemplate](#specistemplate)|boolean||
|[spec.options](#specoptions)|object||
|[spec.patchFrom](#specpatchfrom)|object||
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
|[isTemplate](#specistemplate)|boolean||
|[options](#specoptions)|object||
|[patchFrom](#specpatchfrom)|object||
|[realmRef](#specrealmref)|string|✅|

the KeycloakClientScope resource

---

### spec.definition

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[attributes](#specdefinitionattributes)|object||
|[description](#specdefinitiondescription)|string||
|[id](#specdefinitionid)|string||
|[name](#specdefinitionname)|string||
|[protocol](#specdefinitionprotocol)|string||
|[protocolMappers[]](#specdefinitionprotocolmappers)|object||

&nbsp;

|Validation Rule|Error Message|
|:--------------|:------------|
|has(self.id) == has(oldSelf.id)|Value is immutable|

*missing*

---

### spec.definition.attributes

Type: object

*missing*

---

### spec.definition.description

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

### spec.definition.name

Type: string

*missing*

---

### spec.definition.protocol

Type: string

*missing*

---

### spec.definition.protocolMappers[]

Type: object

|Property|Type|Required|
|:-------|:---|:------:|
|[config](#specdefinitionprotocolmappersconfig)|object||
|[consentRequired](#specdefinitionprotocolmappersconsentrequired)|boolean||
|[consentText](#specdefinitionprotocolmappersconsenttext)|string||
|[id](#specdefinitionprotocolmappersid)|string||
|[name](#specdefinitionprotocolmappersname)|string||
|[protocol](#specdefinitionprotocolmappersprotocol)|string||
|[protocolMapper](#specdefinitionprotocolmappersprotocolmapper)|string||

*missing*

---

### spec.definition.protocolMappers[].config

Type: object

*missing*

---

### spec.definition.protocolMappers[].consentRequired

Type: boolean

*missing*

---

### spec.definition.protocolMappers[].consentText

Type: string

*missing*

---

### spec.definition.protocolMappers[].id

Type: string

*missing*

---

### spec.definition.protocolMappers[].name

Type: string

*missing*

---

### spec.definition.protocolMappers[].protocol

Type: string

*missing*

---

### spec.definition.protocolMappers[].protocolMapper

Type: string

*missing*

---

### spec.isTemplate

Type: boolean

*missing*

---

### spec.options

Type: object

Options for the request to the Keycloak Admin API.

---

### spec.patchFrom

Type: object

Defines additional values that can be loaded from secrets or configmaps. Field selectors are not supported. For more informations see [the patches documentation](../configuration/patches.md).

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