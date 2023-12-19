def insert_dash(num):
    digits = [int(x) for x in str(num)]
    value = ""
    for j in range(len(digits)):
        if j > 0 and  digits[j] % 2 != 0 and digits[j - 1] % 2 != 0:
            value += "-"
        value += str(digits[j])
    return value   