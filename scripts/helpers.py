from itertools import groupby

def bitsep(val):
    if type(val) is int:
        bits = "{0:064b}".format(val)
    else:
        bits = val

    return '_'.join(''.join(x[1] for x in elem[1]) for elem in groupby(enumerate(bits), key=lambda i: i[0] // 4))
