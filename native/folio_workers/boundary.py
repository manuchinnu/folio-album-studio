"""Pydantic models generated at runtime from the canonical TS JSON Schema.

Only the deliberately bounded schema subset used by the protocol is supported.
Unknown schema constructs fail rather than silently dropping constraints.
"""
import json
from pathlib import Path
from typing import Any, Literal, Union
from pydantic import ConfigDict, Field, create_model
from jsonschema import Draft7Validator

SCHEMA_PATH = Path(__file__).resolve().parents[2] / 'packages/core/generated/request.schema.json'
SCHEMA = json.loads(SCHEMA_PATH.read_text(encoding='utf-8'))

def compile_type(s: dict, name: str):
    if 'const' in s:
        return Literal[s['const']]
    if 'anyOf' in s:
        return Union[tuple(compile_type(v, f'{name}_{i}') for i, v in enumerate(s['anyOf']))]
    kind = s.get('type')
    if kind == 'object':
        fields = {}
        for key, value in s['properties'].items():
            constraints = {}
            for source, dest in [('minimum','ge'),('maximum','le'),('minLength','min_length'),('maxLength','max_length'),('minItems','min_length'),('maxItems','max_length'),('pattern','pattern')]:
                if source in value: constraints[dest] = value[source]
            fields[key] = (compile_type(value, name+'_'+key), Field(..., **constraints))
        return create_model(name, __config__=ConfigDict(strict=True, extra='forbid', allow_inf_nan=False), **fields)
    if kind == 'array': return list[compile_type(s['items'], name+'_item')]
    if kind == 'null': return type(None)
    if kind in ('number','integer','string','boolean'):
        return {'number':float,'integer':int,'string':str,'boolean':bool}[kind]
    raise ValueError(f'Unsupported schema construct: {s}')

Request = compile_type(SCHEMA, 'Request')

def validate_request(data: Any):
    Draft7Validator(SCHEMA).validate(data)
    return Request.model_validate(data).model_dump()
