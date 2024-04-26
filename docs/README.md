@canonicxyz/runes

# @canonicxyz/runes

## Table of contents

### Functions

- [parseRunestone](README.md#parserunestone)
- [verifyRunestone](README.md#verifyrunestone)

## Functions

### parseRunestone

▸ **parseRunestone**(`tx`): `any`

Parse a TX and return a runestone message. Throws if the TX does not contain a runestone.

#### Parameters

| Name | Type |
| :------ | :------ |
| `tx` | `Uint8Array` |

#### Returns

`any`

#### Defined in

runes.d.ts:8

___

### verifyRunestone

▸ **verifyRunestone**(`tx`): `boolean`

Verify if a TX is valid if:
* TX does not contain a runestone OR
* TX contains a runestone and contains no cenotaphs

#### Parameters

| Name | Type |
| :------ | :------ |
| `tx` | `Uint8Array` |

#### Returns

`boolean`

#### Defined in

runes.d.ts:17
