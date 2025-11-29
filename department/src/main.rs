#[derive(Debug)]
struct Department {
    name: String,
    lecturers: Vec<Lecturer>,
    students: Vec<Student>,
    courses: Vec<Course>,
}

#[derive(Debug)]

struct Course {
    title: String,
    code: String,
    unit_load: i16,
}
#[derive(Debug)]

struct Lecturer {
    name: String,
    courses: Vec<Course>,
    has_phd: bool,
    has_master: bool,
}
#[derive(Debug)]
struct Student {
    name: String,
    courses: Vec<Course>,
    cgpa: f32,
}

impl Department {
    fn new(name: String) -> Self {
        Department {
            name,
            lecturers: vec![],
            students: vec![],
            courses: vec![],
        }
    }

    fn add_course(&mut self, course: Course) {
        self.courses.push(course);
    }
}

impl Course {
    fn new(title: String, code: String, unit_load: i16) -> Self {
        Course {
            title,
            code,
            unit_load,
        }
    }
}

impl Lecturer {
    fn new(name: String, has_phd: bool, has_master: bool) -> Self {
        Lecturer {
            name,
            courses: vec![],
            has_master,
            has_phd,
        }
    }
}

impl Student {
    fn new(name: String) -> Self {
        Student {
            name,
            courses: vec![],
            cgpa: 0.0,
        }
    }

    fn add_course(&mut self, course: Course) {
        self.courses.push(course);
    }
}

fn main() {
    let mut department = Department::new(String::from("Computer Science"));
    let lecturer_one = Lecturer::new(String::from("Mr Izu"), false, true);
    let mut student_one = Student::new(String::from("Henry Nnamani"));
    let course_one = Course::new(
        String::from("Introduction to Computer Programming"),
        String::from("COS241"),
        3,
    );

    let course_two = Course::new(
        String::from("Physics for Physical Sciences"),
        String::from("PHY116"),
        2,
    );

    department.add_course(course_one);
    student_one.add_course(course_one);
}
