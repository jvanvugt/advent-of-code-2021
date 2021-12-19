import copy
import dataclasses


@dataclasses.dataclass
class Node:
    type_: str
    num: int = None
    left: "Node" = None
    right: "Node" = None

    def __str__(self):
        if self.type_ == "num":
            return str(self.num)
        return f"[{str(self.left)},{str(self.right)}]"

def add(left: Node, right: Node) -> Node:
    result = Node("pair", left=copy.deepcopy(left), right=copy.deepcopy(right))
    while reduce(result):
        pass
    return result

def make_tree(number) -> Node:
    if isinstance(number, int):
        return Node(type_="num", num=number)
    return Node(
        type_="pair",
        left=make_tree(number[0]),
        right=make_tree(number[1]),
    )

def magnitude(node) -> int:
    if node.type_ == "num":
        return node.num
    return 3 * magnitude(node.left) + 2 * magnitude(node.right)

def make_ltr_order(node, order):
    if node.type_ == "num":
        order.append(node)
    else:
        make_ltr_order(node.left, order)
        make_ltr_order(node.right, order)

def get_node_idx(node, ltr_order) -> int:
    for i, n in enumerate(ltr_order):
        if node is n:
            return i
    assert False

def explode(node, ltr_order, depth=0) -> bool:
    if node.type_ == "num":
        return False
    if depth == 4:
        left_idx = get_node_idx(node.left, ltr_order)
        right_idx = get_node_idx(node.right, ltr_order)
        if left_idx > 0:
            ltr_order[left_idx-1].num += node.left.num
        if right_idx < len(ltr_order) - 1:
            ltr_order[right_idx+1].num += node.right.num
        node.type_ = "num"
        node.left = None
        node.right = None
        node.num = 0
        return True
    else:
        return explode(node.left, ltr_order, depth + 1) or explode(node.right, ltr_order, depth + 1)

def split(node) -> bool:
    if node.type_ == "pair":
        return split(node.left) or split(node.right)
    elif node.type_ == "num" and node.num < 10:
        return False
    node.type_ = "pair"
    node.left = Node("num", num=node.num // 2)
    node.right = Node("num", num=(node.num+1) // 2)
    node.num = None
    return True

def reduce(number) -> bool:
    ltr_order = []
    make_ltr_order(number, ltr_order)
    return explode(number, ltr_order) or split(number)

def find_largest_pair(nodes):
    largest = 0
    for i, a in enumerate(nodes):
        for j, b in enumerate(nodes):
            if i != j:
                largest = max(magnitude(add(a, b)), largest)
    return largest


nodes = [make_tree(eval(l)) for l in open("inputs/day18.txt").read().splitlines()]
result = nodes[0]
for num in nodes[1:]:
    result = add(result, num)
print("Part 1:", magnitude(result))
print("Part 2:", find_largest_pair(nodes))
