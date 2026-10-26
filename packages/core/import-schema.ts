import {Type,type Static,type TSchema} from '@sinclair/typebox';
import {Value} from '@sinclair/typebox/value';
const object=<T extends Record<string,any>>(properties:T)=>Type.Object(properties,{additionalProperties:false});
const text=Type.String({maxLength:4096});
const nullable=<T extends TSchema>(s:T)=>Type.Union([s,Type.Null()]);
export const ImportRequestSchema=object({protocolVersion:Type.Literal(1),paths:Type.Array(Type.String({minLength:1,maxLength:1024}),{maxItems:3000}),proxySize:Type.Integer({minimum:128,maximum:1024})});
export const InspectedImageSchema=object({
 relativePath:text,sha256:Type.String({pattern:'^[a-f0-9]{64}$'}),format:Type.Union(['JPEG','PNG','WEBP','HEIF'].map(v=>Type.Literal(v))),
 width:Type.Integer({minimum:1,maximum:30000}),height:Type.Integer({minimum:1,maximum:30000}),orientation:Type.Integer({minimum:1,maximum:8}),bytes:Type.Integer({minimum:1}),mtimeSeconds:Type.Number(),proxyPath:text,
 iccSha256:nullable(Type.String({pattern:'^[a-f0-9]{64}$'})),colorAssumption:Type.Union([Type.Literal('embedded-icc'),Type.Literal('assumed-srgb')]),
 captureTime:nullable(text),captureOffset:nullable(text),camera:nullable(text),gps:nullable(object({latitude:Type.Number({minimum:-90,maximum:90}),longitude:Type.Number({minimum:-180,maximum:180}),altitude:nullable(Type.Number())})),
 dhash:Type.String({pattern:'^[a-f0-9]{16}$'}),phash:Type.String({pattern:'^[a-f0-9]{16}$'}),meanLuma:Type.Number({minimum:0,maximum:255}),detail:Type.Number({minimum:0}),warnings:Type.Array(text),
 xmp:Type.Array(object({source:Type.Union([Type.Literal('embedded'),Type.Literal('sidecar')]),sourceFile:text,fields:Type.Array(object({name:text,raw:text}),{maxItems:100})}),{maxItems:2})
});
export const ImportResponseSchema=object({protocolVersion:Type.Literal(1),images:Type.Array(InspectedImageSchema,{maxItems:3000}),errors:Type.Array(object({relativePath:text,message:text}),{maxItems:3000})});
export type ImportRequest=Static<typeof ImportRequestSchema>;
export type ImportResponse=Static<typeof ImportResponseSchema>;
export type InspectedImage=Static<typeof InspectedImageSchema>;
export function parseImportResponse(value:unknown):ImportResponse{if(!Value.Check(ImportResponseSchema,value))throw new Error([...Value.Errors(ImportResponseSchema,value)].map(e=>e.path+': '+e.message).join('; '));return value;}
