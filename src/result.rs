#[derive(Debug, Copy, Clone)]
enum ProtectedLocation {
    All,
    Office,
    Warehouse,
}

impl ProtectedLocation {
    fn required_access_level(&self) -> u16 {
        match self {
            Self::All => 1000,
            Self::Office => 800,
            Self::Warehouse => 500,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum AuthorizationStatus {
    Allow,
    Deny,
}

#[derive(Debug)]
struct KeyCard {
    access_level: u16,
}

#[derive(Clone, Debug)]
struct Employee {
    name: String,
}

impl Employee {
    fn get_keycard(&self, employee: &Employee) -> Result<KeyCard, String> {
        match employee.name.as_str() {
            "Antia" => Ok(KeyCard { access_level: 1000 }),
            "Brody" => Ok(KeyCard { access_level: 500 }),
            other => Err(format!("{other} doesn't have a keycard")),
        }
    }
}

#[derive(Debug)]
struct Database;
impl Database {
    fn connect() -> Result<Self, String> {
        Ok(Database)
    }

    fn find_employee(&self, name: &str) -> Result<Employee, String> {
        match name {
            "Anita" => Ok(Employee {
                name: "Anita".to_string(),
            }),
            "Brody" => Ok(Employee {
                name: "Brody".to_string(),
            }),
            "Catherine" => Ok(Employee {
                name: "Catherine".to_string(),
            }),
            _ => Err(String::from("Employee not found")),
        }
    }
}

fn authorize(
    employee_name: &str,
    location: ProtectedLocation,
) -> Result<AuthorizationStatus, String> {
    // Step1: connect to database
    let db = Database::connect()?;
    // Step2: Find the employee with the find_employee database function
    let employee = db.find_employee(employee_name)?;
    // Step3: Get a keycard with the `get_keycard` database function
    let keycard = employee.get_keycard(&employee)?;
    // Step4: Determine if the keycard's `access_level` is sufficient, using the
    //   `required_access_level` function implemented on `ProtectedLocation`.
    //   * Higer `access_level` values grant more access to `ProtectedLocation`
    //   1000 can access 1000 and lower. 800 can access 500 but not 1000, ...

    if keycard.access_level >= location.required_access_level() {
        Ok(AuthorizationStatus::Allow)
    } else {
        Ok(AuthorizationStatus::Deny)
    }
}

fn main() {
    // Anita is trying to access the Warehouse, which requires access level 500.
    // His keycard has access level 1000, which should be denied.
    let anita_authorized = authorize("Anita", ProtectedLocation::Warehouse);

    // Brody is trying to access the Office, which requires access level 800.
    // His keycard has access level 500, which should be denied.
    let brody_authorized: Result<AuthorizationStatus, String> =
        authorize("Brody", ProtectedLocation::Office);
    let catherine_authorized: Result<AuthorizationStatus, String> =
        authorize("Catherine", ProtectedLocation::Office);

    //
}
