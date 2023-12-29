// struct for GPA
struct PositiveFloat(f64);

// implement method for creating a positive float
// this will assign the method to the struct Positive Float
impl PositiveFloat {
    pub fn new(value: f64) -> Option<PositiveFloat> {
        if value >= 0.0 {
            Some(PositiveFloat(value))
        }
        else {
            None
        }
    }
    // getter
    pub fn value(&self) -> f64 {
        self.0 // accesses first (and only) field of the PositiveFloat struct
    }
}

// define a struct called Student
struct Student {
    fname: String,
    lname: String,
    year: String,
    gpa: PositiveFloat,
}

// define another struct called Teacher
struct Teacher {
    fname: String,
    lname: String,
    subject: String,
    salary: i32,
    years_experience: i32,
    expected_pay_raise: bool,
}

// lets create a method where Teacher can change a student's GPA
// void return type as we are simply modifying the existing student
impl Teacher {
    pub fn change_student_gpa(&self, student: &mut Student, new_gpa: PositiveFloat) {
        student.gpa = new_gpa;
    }
}

pub fn school_scenario() {
    // create the student
    let mut student = Student {
        fname: String::from("Stanley"),
        lname: String::from("Yelnats"),
        year: String::from("Senior"),
        gpa: PositiveFloat::new(3.2).expect("GPA must be positive."),
    };

    // create the teacher
    let teacher = Teacher {
        fname: String::from("Terry"),
        lname: String::from("Davis"),
        subject: String::from("Computer Science"),
        salary: i32::from(145000),
        years_experience: i32::from(20),
        expected_pay_raise: bool::from(true),
    };

    println!("Student {} {} has entered the room.", student.fname, student.lname);
    println!("This guy is a {} and currently has a GPA of {}", student.year, student.gpa.value());
    println!("Ah, but Teacher {} {} has just arrived with the exams!", teacher.fname, teacher.lname);
    println!("He has been around awhile (for about {} years) and is currently the head teacher of {}", teacher.years_experience, teacher.subject);
    let new_gpa: PositiveFloat = PositiveFloat::new(2.9).expect("GPA error!");
    teacher.change_student_gpa(&mut student, new_gpa);
    println!("Now with the exams graded, {} only has a {} GPA", student.fname, student.gpa.value());
}