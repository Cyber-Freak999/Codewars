def break_camel_case(s):
    """Break camelCase string into words"""
    return "".join([" " + char if char.isupper() else char for char in s])
