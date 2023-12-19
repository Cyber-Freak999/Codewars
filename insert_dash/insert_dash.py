def insert_dash(num):
    """Insert dashes between each two odd numbers in num."""
    digits = [int(x) for x in str(num)]
    value = ""
    for j, digit in enumerate(digits):
        if j > 0 and digit % 2 != 0 and digits[j - 1] % 2 != 0:
            value += "-"
        value += str(digit)
    return value
