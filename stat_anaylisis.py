def analysis(s):
    assert(type(s) == bytes)
    a = [0]*128
    for i in s:
        a[i] += 1
    return a

def trim_vector(a):
    assert(type(a) == list)
    x = []
    for i in a:
        if i != 0:
            x.append((chr(i), i))
    return x

if __name__ == "__main__":
    message = open("message.txt", 'r').read()

    a = analysis(message)
    print(a)
    a = trim_vector(a)
    print(a)