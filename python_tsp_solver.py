import itertools as it
from dataclasses import dataclass


@dataclass
class Edge():
    n1: int
    n2: int
    ni1: int
    ni2: int
    cost: float


def solve(graph: list[tuple[float, float]]):
    cost = create_cost(graph)
    route = greedy(cost)
    two_opt(cost, route)

    return route


def create_cost(graph: list[tuple[float, float]]):
    cost = [[0 for _ in range(len(graph))] for _ in range(len(graph))]

    for i, p1 in enumerate(graph):
        for jj, p2 in enumerate(graph[i+1:]):
            j = jj + i + 1
            distance = ((p1[0] - p2[0])**2. + (p1[1] - p2[1])**2.)**0.5
            cost[i][j] = distance
            cost[j][i] = distance
    
    return cost


def greedy(cost: list[list[float]]):
    remaining = [i for i in range(len(cost))]
    current = remaining.pop(0)
    route = [0]

    while len(remaining) > 0:
        next = calc_next_node(cost, current, remaining)
        route.append(remaining[next])
        current = route[-1]
        remaining.remove(current)
    
    return route


def calc_next_node(cost, current, remaining):
    min_d = 9999999999
    next = current
    n1 = current

    for i, n2 in enumerate(remaining):
        if cost[n1][n2] < min_d:
            min_d = cost[n1][n2]
            next = i
    
    return next


def two_opt(cost: list[list[float]], route: list[int]):
    cnt = 0
    while True:
        has_improved = False
        edges = get_edges(cost, route)
        for (e1, e2) in it.combinations(edges, 2):
            if e1.n1 == e2.n2 or e2.n1 == e1.n2:
                continue

            org_cost = e1.cost + e2.cost
            new_cost = cost[e1.n1][e2.n1] + cost[e1.n2][e2.n2]

            if new_cost < org_cost:
                update_route(route, e1, e2)
                has_improved = True
                cnt += 1
                break
                
        if not has_improved:
            break
    
    # print(f"two-opt: {cnt}")


def update_route(route: list[int], e1: Edge, e2: Edge):
    n1 = e1.ni1
    n2 = e2.ni1

    route[n1+1:n2+1] = route[n1+1:n2+1][::-1]

        
def get_edges(cost, route) -> list[Edge]:
    edge_list = []
    ni = 0
    for n1, n2 in zip(route[:-1], route[1:]):
        c = cost[n1][n2]
        edge = Edge(n1, n2, ni, ni+1, c)
        edge_list.append(edge)
        ni += 1

    n1 = route[-1]
    n2 = route[0]
    c = cost[n1][n2]
    ni1 = len(route) - 1
    ni2 = 0
    edge = Edge(n1, n2, ni1, ni2, c)
    edge_list.append(edge)

    return edge_list
