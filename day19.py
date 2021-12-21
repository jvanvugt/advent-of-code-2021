import itertools

import numpy as np

reports = open("inputs/day19.txt").read().split("\n\n")
beacons_per_scanner = [
    np.fromstring(",".join(r.split("\n")[1:]), sep=",", dtype=int).reshape(-1, 3)
    for r in reports
]
eye = np.eye(3, dtype=int)
all_rotations = [
    eye[order, :] * signs
    for order in itertools.permutations([0, 1, 2], 3)
    for signs in itertools.product([-1, 1], repeat=3)
]

def try_align(ref, other):
    for rot in all_rotations:
        rot_other = other @ rot.T
        for beacon in rot_other[:-11]:
            for ref_beacon in ref:
                trans = ref_beacon - beacon
                trans_other = rot_other + trans
                trans_other = set(map(tuple, trans_other))
                matches = len(trans_other & ref)
                if matches >= 12:
                    return trans_other, tuple(trans)
    return None

combined = set(map(tuple, beacons_per_scanner[0]))
to_be_aligned = list(range(1, len(beacons_per_scanner)))
scanners = [(0, 0, 0)]*len(beacons_per_scanner)
while len(to_be_aligned) > 0:
    aligned = False
    for i in list(to_be_aligned):
        result = try_align(combined, beacons_per_scanner[i])
        if result is not None:
            transformed, scanner_pos = result
            scanners[i] = scanner_pos
            combined |= transformed
            print("Aligned:", i)
            to_be_aligned.remove(i)
            aligned = True
    assert aligned

max_distance = 0
for i, a in enumerate(scanners):
    for b in scanners[i+1:]:
        max_distance = max(max_distance, sum(abs(x - y) for x, y in zip(a, b)))

print("Part 1:", len(combined))
print("Part 2:", max_distance)
