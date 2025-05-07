#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, vec, Address, Env, String, Vec};


// Hospital contract Outline 

// Admin 

// Patient Managment Functions 
// -> register a patient 
// -> get a patient information 
// -> update patient record 
// -> set patient active 
// -> list all patients 

// Doctor management Functions 
// -> register a doctor 
// -> get Doctor Information 
// -> update Doctor information 
// -> set doctor active 
// -> list all Doctors 

// Mecdical Test Management functions 
// -> record medical test 
// -> get medical test (Doctor and patient) 
// -> get all medical test for a patient 
// -> get all medical test performed by a doctor 
// -> Statistics records for test and which department requested for test 
// -> list of medical tests 

// Things to take note 
// Structure of data  
// How to retreive data and store data 
// How to get environment variables 



#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Patient {
    id: u64,
    name: String,
    date_of_birth: u64,
    blood_type: String,
    allergies: Vec<String>,
    insurance_id: String,
    active: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Doctor {
    id: u64,
    name: String,
    specialization: String,
    license_number: String, 
    active: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MedicalTest {
    id: u64,
    patient_id: u64,
    doctor_id: u64,
    test_type: String,
    test_date: u64,
    results: String,
    notes: String,
}
 

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Admin,
    Patient(u64),
    Doctor(u64),
    MedicalTest(u64),
    PatientTests(u64),
    DoctorTests(u64),
    PatientCount,
    DoctorCount,
    TestCount,
}


#[contract]
pub struct HospitalContract;

#[contractimpl]
impl HospitalContract {

    // Initializer -> It initialize the contract with the admin 
   pub fn initialize(env: Env, admin: Address) -> Address {

    if env.storage().instance().has(&DataKey::Admin) {
        panic!("Contract already initilized");
    }

    env.storage().instance().set(&DataKey::Admin, &admin);
    env.storage().instance().set(&DataKey::PatientCount, &0u64);
    env.storage().instance().set(&DataKey::DoctorCount, &0u64);
    env.storage().instance().set(&DataKey::TestCount, &0u64);

    admin
   }

   fn check_admin (env: &Env) {
    let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
    admin.require_auth();
   }

   // Patient Managment Functions 

   pub fn register_patient (
    env: Env,
    name: String,
    date_of_birth: u64,
    blood_type: String,
    allergies: Vec<String>,
    insurance_id: String,
   ) -> u64 {
        Self::check_admin(&env);

        // Get and increment patient count
        let patient_count: u64 = env.storage().instance().get(&DataKey::PatientCount).unwrap_or(0);
        let new_id = patient_count + 1;

        // create the patiet record 
        let patient = Patient {
            id: new_id,
            name,
            date_of_birth,
            blood_type,
            allergies,
            insurance_id,
            active: true,
        };
        
        // Store the patient data and update update the count 

        env.storage().instance().set(&DataKey::Patient(new_id), &patient);
        env.storage().instance().set(&DataKey::PatientCount, &new_id);

        // Initialize empty test list for patient 

        env.storage().instance().set(&DataKey::PatientTests(new_id), &Vec::<u64>::new(&env));

        new_id
   }

   // Get patient information 

   pub fn get_patient(env: Env, id: u64) -> Patient {
        match env.storage().instance().get(&DataKey::Patient(id)) {
            Some(patient) => patient,
            None => panic!("Patient not registered"),
        }
   }


   // Update patient record 
   pub fn update_patient (
    env: Env,
    id: u64,
    name: String,
    date_of_birth: u64,
    blood_type: String,
    allergies: Vec<String>,
    insurance_id: String
   ) -> Patient {
    Self::check_admin(&env);

    // Get existing patient 
    let mut patient: Patient = env.storage().instance().get(&DataKey::Patient(id)).
        unwrap_or_else(|| panic!("Patient not found"));


    // Update fields 
    patient.name = name;
    patient.date_of_birth = date_of_birth;
    patient.blood_type = blood_type;
    patient.allergies = allergies;
    patient.insurance_id = insurance_id;

    // Save the updated patient 
    env.storage().instance().set(&DataKey::Patient(id), &patient);

    patient
    
   }

   pub fn set_patient_active(env: Env, id: u64, active: bool) -> Patient {

    Self::check_admin(&env);

    // Get existing patient 
    let mut patient: Patient = env.storage().instance().get(&DataKey::Patient(id)).unwrap_or_else(|| panic!("Patient not found"));

    // Update status 
    patient.active = active;

    // Save updated patient 
    env.storage().instance().set(&DataKey::Patient(id), &patient);

    patient
   }

   // List all patients 
   pub fn list_patients(env: Env) -> Vec<Patient> {
    let patient_count: u64 = env.storage().instance().get(&DataKey::PatientCount).unwrap_or(0);
    let mut patients = Vec::new(&env);

    for i in 1..=patient_count {
        if let Some(patient) = env.storage().instance().get(&DataKey::Patient(i)) {
            patients.push_back(patient);
        }
    }
    patients
   }

   pub fn register_doctor(
    env: Env,
    name: String,
    specialization: String,
    license_number: String,
   ) -> u64 {
    Self::check_admin(&env);

    let doctor_count: u64 = env.storage().instance().get(&DataKey::DoctorCount).unwrap_or(0);
    let new_id = doctor_count + 1;

    let doctor = Doctor {
        id: new_id,
        name,
        specialization,
        license_number,
        active: true,
    };

    env.storage().instance().set(&DataKey::Doctor(new_id), &doctor);
    env.storage().instance().set(&DataKey::DoctorCount, &new_id);

    env.storage().instance().set(&DataKey::DoctorTests(new_id), &Vec::<u64>::new(&env));
    
    new_id
   }

   pub fn get_doctor(env: Env, id: u64) -> Doctor {
    match  env.storage().instance().get(&DataKey::Doctor(id)){
        Some(doctor) => doctor,
        None => panic!("Doctor not found"),
    }
   }


   pub fn update_doctor(
    env: Env,
    id: u64,
    name: String,
    specialization: String,
    license_number: String
   ) -> Doctor {

    Self::check_admin(&env);

    let mut doctor: Doctor = env.storage().instance().get(&DataKey::Doctor(id)).unwrap_or_else(|| panic!("Doctor not found"));

    // udate fields 
    doctor.name = name;
    doctor.specialization = specialization;
    doctor.license_number = license_number;

    env.storage().instance().set(&DataKey::Doctor(id), &doctor);

    doctor
   }


   pub fn set_doctor_active(env: Env, id: u64, active: bool) -> Doctor {
    Self::check_admin(&env);

    let  mut doctor: Doctor = env.storage().instance().get(&DataKey::Doctor(id)).unwrap_or_else(|| panic!("Doctor not found"));

    doctor.active = active;

    env.storage().instance().set(&DataKey::Doctor(id), &doctor);

    doctor
   }


   pub fn list_doctors(env: Env) -> Vec<Doctor> {
    let doctor_count: u64 = env.storage().instance().get(&DataKey::DoctorCount).unwrap_or(0);
    let mut doctors = Vec::new(&env);

    for i in 1..=doctor_count {
        if let Some(doctor) = env.storage().instance().get(&DataKey::Doctor(i)) {
            doctors.push_back(doctor);
        }
    }

    doctors
   }


   pub fn record_medical_test(
    env: Env,
    patient_id: u64,
    doctor_id: u64,
    test_type: String,
    test_date: u64,
    results: String,
    notes: String,
   ) -> u64 {
    Self::check_admin(&env);

    let medical_test_count: u64 = env.storage().instance().get(&DataKey::TestCount).unwrap_or(0);
    let new_id = medical_test_count + 1;

    let medical_test_records = MedicalTest {
        id: new_id,
        patient_id,
        doctor_id,
        test_type,
        test_date,
        results,
        notes,
    };

    env.storage().instance().set(&DataKey::MedicalTest(new_id), &medical_test_records);
    env.storage().instance().set(&DataKey::TestCount, &new_id);
    
    new_id
   }

   pub fn get_medical_test(env: Env, id: u64) -> MedicalTest {
    match env.storage().instance().get(&DataKey::MedicalTest(id)) {
        Some(medical_test) => medical_test,
        None => panic!("No medical test found"),
    }
    }  

   pub fn get_all_patient_medical_records(env: Env, patient_id: u64) -> MedicalTest {
    match env.storage().instance().get(&DataKey::MedicalTest(patient_id)) {
        Some(medical_test) => medical_test,
        None => panic!("No medical test record found"),
    }
    }  

   pub fn get_all_doctor_medical_records(env: Env, doctor_id: u64) -> MedicalTest {
    match env.storage().instance().get(&DataKey::MedicalTest(doctor_id)) {
        Some(medical_test) => medical_test,
        None => panic!("No medical test record found"),
    }
    }  

    pub fn get_all_medical_test_records(env: Env) -> Vec<MedicalTest> {
        let medical_count: u64 = env.storage().instance().get(&DataKey::TestCount).unwrap_or(0);
        let mut medical_records = Vec::new(&env);
    
        for i in 1..=medical_count {
            if let Some(_medical_records) = env.storage().instance().get(&DataKey::MedicalTest(i)) {
                medical_records.push_back(_medical_records);
            }
        }
        medical_records
       }

}



mod test;
