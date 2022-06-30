mod data {
  use std::collections::VecDeque;

  #[derive(Debug)]
  pub struct Vertex {
    num: i32,
    edges: Vec<i32>,
    degrees: i32,
  }

  #[derive(Debug)]
  pub struct Graph {
    vertices: Vec<Vertex>,
  }

  impl Graph {
    pub fn new(vertices: i32) -> Self {
      Self {
        vertices: (0..vertices)
          .map(|i| Vertex {
            num: i,
            edges: vec![],
            degrees: 0,
          })
          .collect::<Vec<_>>(),
      }
    }

    pub fn add_edge(&mut self, edge_head: i32, edge_tail: i32) {
      if let Some(v) = self.vertices.get_mut(edge_head as usize) {
        v.degrees += 1;
      }
      self
        .vertices
        .get_mut(edge_tail as usize)
        .map(|vt| vt.edges.push(edge_head))
        .expect("Graph not initialized")
    }

    pub fn topological_sort(&self) -> Vec<i32> {
      let mut ordered_vertices = vec![];
      let mut degree_count =
        self.vertices.iter().map(|v| v.degrees).collect::<Vec<_>>();

      let mut sources = self
        .vertices
        .iter()
        .rev()
        .filter(|v| v.degrees == 0)
        .collect::<VecDeque<_>>();

      while let Some(v) = sources.pop_front() {
        ordered_vertices.push(v);
        for &edge in &v.edges {
          if let Some(d) = degree_count.get_mut(edge as usize) {
            *d -= 1;
            if *d == 0 {
              if let Some(child_vertex) = self.vertices.get(edge as usize) {
                sources.push_back(child_vertex);
              }
            }
          }
        }
      }

      ordered_vertices.iter().map(|v| v.num).collect()
    }
  }
}

use data::Graph;

pub fn find_order_dag(
  num_courses: i32,
  prerequisites: Vec<Vec<i32>>,
) -> Vec<i32> {
  // We have no directed graph so just return a sequential course list.
  if prerequisites.is_empty() {
    return (0..num_courses).collect();
  }

  let mut graph = Graph::new(num_courses);
  for p in prerequisites {
    let &edge_head = p.get(0).expect("never empty");
    let &edge_tail = p.get(1).expect("never empty");
    graph.add_edge(edge_head, edge_tail);
  }
  let sort = graph.topological_sort();
  // Check if we have a cyclic graph.
  if sort.len() != num_courses as usize {
    return vec![];
  }
  sort
}

#[cfg(test)]
mod test {
  use crate::course_schedule_dag::find_order_dag as find_order;

  #[test]
  fn test_find_order_dfs() {
    assert_eq!(
      vec![2, 1, 0],
      find_order(3, vec![vec![0, 1], vec![0, 2], vec![1, 2]])
    );
    assert_eq!(vec![0, 1, 2], find_order(3, vec![vec![2, 1], vec![1, 0]]));
    assert_eq!(vec![1, 0, 2], find_order(3, vec![vec![2, 0], vec![2, 1]]));
    assert_eq!(vec![0, 1], find_order(2, vec![vec![1, 0]]));
    assert_eq!(
      vec![0, 1, 2, 3],
      find_order(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]])
    );
    assert_eq!(vec![0], find_order(1, vec![]));
    assert_eq!(vec![0, 1], find_order(2, vec![]));
    assert_eq!(
      vec![] as Vec<i32>,
      find_order(2, vec![vec![0, 1], vec![1, 0]])
    );
    assert_eq!(vec![2, 0, 1], find_order(3, vec![vec![1, 0]]));
  }
}
