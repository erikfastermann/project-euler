triangles = []
with open('p102_triangles.txt') as f:
    for line in f:
        split = line.split(',')
        current = []
        for i in range(0, 6, 2):
            current.append((int(split[i]), int(split[i+1])))
        triangles.append(current)

# https://stackoverflow.com/questions/2049582/how-to-determine-if-a-point-is-in-a-2d-triangle

def sign(p1, p2, p3):
    return (p1[0] - p3[0]) * (p2[1] - p3[1]) - (p2[0] - p3[0]) * (p1[1] - p3[1])

def point_in_triangle(pt, v1, v2, v3):
    d1 = sign(pt, v1, v2)
    d2 = sign(pt, v2, v3)
    d3 = sign(pt, v3, v1)

    has_neg = (d1 < 0) or (d2 < 0) or (d3 < 0)
    has_pos = (d1 > 0) or (d2 > 0) or (d3 > 0)

    return not (has_neg and has_pos)

contains_origin = 0
for tri in triangles:
    if point_in_triangle((0, 0), tri[0], tri[1], tri[2]):
        contains_origin += 1

print(contains_origin)