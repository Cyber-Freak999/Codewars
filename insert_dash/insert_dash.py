# Write a function insert_dash(num) / insertDash(num) / InsertDash(int num) 
# that will insert dashes ('-') between each two odd digits in num. 
# For example: if num is 454793 the output should be 4547-9-3. 
# Don't count zero as an odd digit.

def insert_dash(num):
    digits = [int(x) for x in str(num)]
    value = ""
    for j in range(len(digits)):
        if j > 0 and  digits[j] % 2 != 0 and digits[j - 1] % 2 != 0:
            value += "-"
        value += str(digits[j])
    return value   