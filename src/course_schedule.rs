/// Initial attempt not having looked at solutions. After reviewing solution will
/// need to update using https://www.geeksforgeeks.org/topological-sorting/

mod data {
  use std::{cmp::Ordering, iter::from_fn};

  #[derive(Debug, Default)]
  pub struct CourseGraph {
    nodes: Vec<Course>,
  }

  #[derive(Debug, Clone)]
  pub struct Course {
    num: i32,
    prerequisites: Vec<i32>,
    references: Vec<i32>,
  }

  impl Course {
    pub fn course_num(&self) -> &i32 {
      &self.num
    }
    pub fn prerequisites(&self) -> &[i32] {
      self.prerequisites.as_slice()
    }
  }

  impl PartialEq for Course {
    fn eq(&self, other: &Self) -> bool {
      self.num == other.num
    }
  }

  impl Eq for Course {}

  impl PartialOrd for Course {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      Some(self.cmp(other))
    }
  }

  impl Ord for Course {
    fn cmp(&self, other: &Self) -> Ordering {
      self.num.cmp(&other.num)
    }
  }

  impl CourseGraph {
    fn add_course_with_prerequisite(
      &mut self,
      course_num: i32,
      prerequisite_num: i32,
    ) {
      match self.get_course_mut(&course_num) {
        // Add the prerequisite
        Some(c) => {
          c.prerequisites.push(prerequisite_num);
          // If the prerequisites does not exist add it.
          match self.get_course_mut(&prerequisite_num) {
            Some(c) => {
              c.references.push(course_num);
            }
            None => self.nodes.push(Course {
              num: prerequisite_num,
              references: vec![course_num],
              prerequisites: vec![],
            }),
          }
        }
        // find the prerequisite
        None => {
          self.nodes.push(Course {
            num: course_num,
            prerequisites: vec![prerequisite_num],
            references: vec![],
          });
          // update or add the prerequisite.
          match self.get_course_mut(&prerequisite_num) {
            Some(c) => c.references.push(course_num),
            None => self.nodes.push(Course {
              num: prerequisite_num,
              prerequisites: vec![],
              references: vec![course_num],
            }),
          }
        }
      }
    }

    fn get_course_mut(&mut self, course_num: &i32) -> Option<&mut Course> {
      self.nodes.iter_mut().find(|c| &c.num == course_num)
    }

    pub fn get_courses(&self) -> impl Iterator<Item = &Course> {
      self.nodes.iter().filter(|c| !c.prerequisites.is_empty())
    }

    pub fn get_courses_with_no_prerequisites(
      &mut self,
      references: bool,
    ) -> impl Iterator<Item = Course> + '_ {
      from_fn(move || {
        self
          .nodes
          .iter()
          .enumerate()
          .filter(|(_, c)| {
            if references {
              c.prerequisites.is_empty() && !c.references.is_empty()
            } else {
              c.prerequisites.is_empty() && c.references.is_empty()
            }
          })
          .map(|(i, _)| i)
          .next()
          .map(|i| self.nodes.remove(i))
      })
    }

    pub fn len(&self) -> usize {
      self.nodes.len()
    }

    pub fn has_course_num(&self, course_num: &i32) -> bool {
      self.nodes.iter().any(|c| c.course_num() == course_num)
    }

    pub fn add_course(&mut self, course_num: i32) {
      self.nodes.push(Course {
        num: course_num,
        prerequisites: vec![],
        references: vec![],
      })
    }

    pub fn remove_course(&mut self, course_num: &i32) {
      if let Some((i, _)) = self
        .nodes
        .iter()
        .enumerate()
        .find(|(_, c)| c.course_num() == course_num)
      {
        self.nodes.remove(i);
      }
    }

    pub fn is_empty(&self) -> bool {
      self.nodes.is_empty()
    }
  }

  impl From<Vec<Vec<i32>>> for CourseGraph {
    fn from(prerequisites: Vec<Vec<i32>>) -> Self {
      let mut courses = CourseGraph::default();

      for c in prerequisites {
        let course_num = *c.get(0).expect("Never empty");
        let prerequisite_num = *c.get(1).expect("Never empty");
        courses.add_course_with_prerequisite(course_num, prerequisite_num);
      }
      courses.nodes.sort();
      courses
    }
  }
}

use data::{Course, CourseGraph};

pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
  let mut courses = CourseGraph::from(prerequisites);
  // Add missing course number with no prerequisites.
  if courses.len() < num_courses as usize {
    for course_num in 0..num_courses {
      if !courses.has_course_num(&course_num) {
        courses.add_course(course_num)
      }
    }
  }
  dbg!(&courses);
  let mut course_order: Vec<Course> = vec![];

  // Add courses with no prerequisites and no references first.
  for course in courses.get_courses_with_no_prerequisites(false) {
    course_order.push(course);
  }

  // Add courses with no prerequisites but having references.
  for course in courses.get_courses_with_no_prerequisites(true) {
    course_order.push(course);
  }

  // Remove courses that have been selected.
  for course in course_order.iter() {
    courses.remove_course(course.course_num());
  }

  // Iterate over remaining courses attempting to add courses that have prerequisites we've
  // already selected.
  while course_order.len() < num_courses as usize && !courses.is_empty() {
    let selected_course_num = course_order.len();
    for course in courses.get_courses() {
      let selected_courses = course_order.iter().map(|c| c.course_num());
      // Check we have all the prerequisites.
      if course
        .prerequisites()
        .iter()
        .all(|p| selected_courses.clone().any(|course_num| course_num == p))
      {
        course_order.push(course.clone())
      }
    }

    // Check if we found no courses in this iteration. This would be a cycle in the course number and prerequisites.
    if course_order.len() == selected_course_num {
      return vec![];
    }

    // Remove selected courses for next iteration.
    for course in course_order.iter() {
      courses.remove_course(course.course_num());
    }
  }

  // Return the selected courses as a Vec of course reference numbers.
  course_order
    .into_iter()
    .map(|c| *c.course_num())
    .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
  use crate::course_schedule::find_order;

  #[test]
  fn test_find_order() {
    assert_eq!(
      vec![2, 1, 0],
      find_order(3, vec![vec![0, 1], vec![0, 2], vec![1, 2]])
    );
    assert_eq!(vec![0, 1, 2], find_order(3, vec![vec![2, 1], vec![1, 0]]));
    assert_eq!(vec![0, 1, 2], find_order(3, vec![vec![2, 0], vec![2, 1]]));
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
