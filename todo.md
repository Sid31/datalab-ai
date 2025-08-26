# VRAM-Synth: Decentralized Medical Synthetic Data Marketplace (Gretel AI Copycat POC)

**Status**: MVP Foundation Complete | Next: Marketplace Features & Medical Data Processing

This project transforms the VetKeys encrypted notes app into a decentralized synthetic data marketplace inspired by Gretel AI, targeting medical/healthcare data with privacy-preserving features on ICP.

## 🎯 Vision: Gretel AI Copycat on ICP
- **Decentralized Gretel**: Upload → Obfuscate → Marketplace → Purchase/Download workflow
- **Medical Focus**: Healthcare data with PII detection and medical-specific transformations
- **ICP Native**: Leveraging canisters for data sovereignty and immutable audit trails
- **Target Users**: Hospitals, universities, biotech companies, AI researchers

---

## ✅ COMPLETED: Core Infrastructure (Week 1-3)

### Backend (Rust Canister) ✅
- [x] **Principal-based Authentication**: Internet Identity integration with session management
- [x] **Encrypted Storage**: Client-side AES-GCM encryption with vetKeys key derivation
- [x] **Large Dataset Support**: Increased MAX_NOTE_CHARS to 1M characters (handles thousands of rows)
- [x] **Stable Memory**: Persistent storage across canister upgrades (96GB capacity)
- [x] **CSV Data Processing**: JSON parsing and storage of uploaded datasets
- [x] **Access Control**: Owner-only access with sharing capabilities (up to 50 users per dataset)
- [x] **Error Handling**: Proper validation and error messages for upload failures
- [x] **Production Ready**: Deployed and tested with large CSV files (1000+ rows)

### Frontend (Svelte + TypeScript) ✅
- [x] **Modern UI**: Tailwind CSS + DaisyUI with responsive design
- [x] **Drag & Drop Upload**: Intuitive CSV file upload with real-time preview
- [x] **Dataset Management**: Browse, view, and manage uploaded datasets
- [x] **Sample Data Library**: Pre-loaded synthetic datasets (employee, financial, IoT)
- [x] **Authentication Flow**: Complete II integration with session persistence
- [x] **Encryption Service**: Client-side encryption before upload to backend
- [x] **Debug Tools**: Comprehensive logging and state monitoring
- [x] **File Preview**: Row count detection and column header display
- [x] **Professional Sidebar**: Dark theme navigation with VRAM branding
- [x] **Logo Integration**: Dynamic logo switching (full/icon) based on sidebar state
- [x] **Responsive Design**: Mobile-first with collapsible sidebar and overlay
- [x] **Navigation Structure**: Main sections (Upload, Datasets, Samples, Marketplace) + Workspace sections
- [x] **Logo Click-to-Toggle**: Logo itself serves as expand/collapse button (no separate arrow)
- [x] **Optimized Logo Sizing**: 2x larger expanded logo (h-20) for better brand visibility
- [x] **Clean UI**: Removed arrow buttons for cleaner, more intuitive interaction

### Development & Deployment ✅
- [x] **Complete Build System**: dfx deployment with frontend build automation
- [x] **Documentation**: Comprehensive README with installation instructions
- [x] **Local Development**: Working dfx start → deploy → access workflow
- [x] **Environment Setup**: Automatic .env generation with canister IDs
- [x] **Asset Management**: VRAM logo files properly integrated and deployed

---

## 🔲 TODO: Gretel AI Marketplace Features (Next Phase)

### 1. Medical Data Processing Pipeline 🔲
- [ ] **PII Detection Engine**: Implement regex/ML-based detection for medical identifiers
  - [ ] Detect: SSN, medical record numbers, patient IDs, phone numbers, addresses
  - [ ] HIPAA compliance checks for protected health information (PHI)
- [ ] **Medical Data Anonymization**: 
  - [ ] Date shifting (maintain relative relationships)
  - [ ] Demographic generalization (age ranges, zip code truncation)
  - [ ] Medical code preservation (ICD-10, CPT codes)
- [ ] **Data Quality Metrics**: Generate quality scores for synthetic data
  - [ ] Statistical similarity measures
  - [ ] Correlation preservation
  - [ ] Distribution matching scores

### 2. Synthetic Data Generation 🔲
- [ ] **Synthetic Data Engine**: Implement basic synthetic data generation
  - [ ] Statistical sampling methods
  - [ ] Correlation-preserving transformations
  - [ ] Medical-specific generators (vital signs, lab values, demographics)
- [ ] **Generation Parameters**: User controls for synthetic data creation
  - [ ] Privacy level settings (low/medium/high obfuscation)
  - [ ] Sample size selection
  - [ ] Column-specific transformation rules
- [ ] **Validation Pipeline**: Ensure synthetic data quality
  - [ ] Statistical tests against original data
  - [ ] Privacy leakage detection
  - [ ] Medical validity checks

### 3. Marketplace Infrastructure 🔲
- [ ] **Dataset Marketplace**: Public listing of available synthetic datasets
  - [ ] Dataset catalog with metadata (rows, columns, medical specialty)
  - [ ] Search and filtering by medical domain
  - [ ] Preview capabilities (sample rows, statistics)
- [ ] **Pricing & Payment**: ICP-native payment system
  - [ ] ICP token integration for dataset purchases
  - [ ] Pricing models (per-download, subscription, bulk)
  - [ ] Revenue sharing for data contributors
- [ ] **Access Control**: Marketplace-specific permissions
  - [ ] Public/private dataset settings
  - [ ] Purchaser verification and access grants
  - [ ] Download tracking and audit trails

### 4. Advanced Privacy Features 🔲
- [ ] **Differential Privacy**: Add noise injection for stronger privacy guarantees
- [ ] **K-Anonymity**: Ensure minimum group sizes for quasi-identifiers
- [ ] **Synthetic Data Validation**: Privacy leakage testing
- [ ] **Audit Trails**: Immutable logs of all data transformations and access

### 5. Medical-Specific Features 🔲
- [ ] **Medical Data Types**: Specialized handling for healthcare data
  - [ ] EHR (Electronic Health Records) format support
  - [ ] FHIR (Fast Healthcare Interoperability Resources) integration
  - [ ] Medical imaging metadata (DICOM headers)
- [ ] **Compliance Tools**: Healthcare regulation compliance
  - [ ] HIPAA compliance reporting
  - [ ] GDPR compliance for EU medical data
  - [ ] Audit reports for regulatory submissions
- [ ] **Medical Validation**: Domain-specific quality checks
  - [ ] Medical code validation (ICD-10, SNOMED CT)
  - [ ] Clinical range validation (vital signs, lab values)
  - [ ] Temporal consistency checks (treatment sequences)

### 6. Enhanced UI/UX 🔲
- [ ] **Marketplace Interface**: Browse and purchase synthetic datasets
  - [ ] Dataset catalog with search/filter
  - [ ] Shopping cart and checkout flow
  - [ ] Purchase history and download management
- [ ] **Data Processing Dashboard**: Monitor synthetic data generation
  - [ ] Progress tracking for data processing jobs
  - [ ] Quality metrics visualization
  - [ ] Privacy analysis reports
- [ ] **Admin Panel**: Marketplace management tools
  - [ ] Dataset approval workflow
  - [ ] User management and verification
  - [ ] Revenue analytics and reporting

---

## 🎯 3-Day Hackathon MVP Priorities

### Day 1: Medical Data Processing
- [ ] Basic PII detection (SSN, phone, email patterns)
- [ ] Simple anonymization (hash IDs, date shifting)
- [ ] Medical data validation (basic range checks)

### Day 2: Marketplace Core
- [ ] Public dataset listing page
- [ ] Basic ICP payment integration
- [ ] Dataset purchase and download flow

### Day 3: Demo Polish
- [ ] Sample medical datasets (cardiac, lab results, demographics)
- [ ] End-to-end demo workflow
- [ ] Presentation materials and demo script

---

## 🏗️ Technical Architecture

### Current Stack ✅
- **Backend**: Rust + IC CDK + vetKeys + Stable Structures
- **Frontend**: Svelte + TypeScript + Tailwind CSS + DaisyUI
- **Storage**: ICP Stable Memory (persistent, 96GB capacity)
- **Auth**: Internet Identity (decentralized, passwordless)
- **Crypto**: vetKeys (BLS12-381) + AES-GCM encryption

### Planned Additions 🔲
- **Payment**: ICP Ledger integration for marketplace transactions
- **ML/AI**: Basic statistical synthetic data generation
- **Medical**: FHIR/HL7 parsing libraries
- **Privacy**: Differential privacy and k-anonymity libraries

---

## 🚀 Current Status Summary

**✅ Foundation Complete (90%)**:
- Secure CSV upload and storage ✅
- Dataset management UI ✅
- Authentication and encryption ✅
- Large file support ✅
- Professional UI/UX with sidebar navigation ✅
- VRAM branding and logo integration ✅
- Documentation and deployment ✅

**🔲 Gretel.ai Copycat Features Missing (Critical for MVP)**:

### **Core Gretel.ai Features We Need:**
1. **Data Profiling & Analysis** (Gretel's data discovery)
   - [ ] Statistical analysis of uploaded datasets
   - [ ] Data quality scoring and recommendations
   - [ ] Column type detection (PII, sensitive data, categorical, numerical)

2. **Privacy & Anonymization** (Gretel's privacy engineering)
   - [ ] PII detection and masking
   - [ ] K-anonymity and l-diversity implementation
   - [ ] Differential privacy noise injection
   - [ ] Privacy risk assessment scores

3. **Synthetic Data Generation** (Gretel's core feature)
   - [ ] Statistical synthetic data generation
   - [ ] Deep learning-based synthetic models
   - [ ] Correlation preservation algorithms
   - [ ] Quality metrics (statistical similarity, utility preservation)

4. **Marketplace & Monetization** (Our ICP advantage)
   - [ ] Public dataset catalog
   - [ ] ICP token payment system
   - [ ] Revenue sharing for data contributors
   - [ ] Dataset licensing and access controls

5. **Enterprise Features** (Gretel's business model)
   - [ ] API access for programmatic usage
   - [ ] Batch processing for large datasets
   - [ ] Compliance reporting (HIPAA, GDPR)
   - [ ] Team collaboration and workspace management

**Next Critical Milestone**: Implement data profiling and PII detection to match Gretel's data discovery capabilities.

---

## 🎯 Gretel.ai Feature Parity Roadmap

### **Phase 1: Data Intelligence (Week 4)**
- [ ] Statistical profiling engine
- [ ] PII detection patterns
- [ ] Data quality scoring
- [ ] Privacy risk assessment

### **Phase 2: Synthetic Generation (Week 5-6)**
- [ ] Basic statistical synthetic data generation
- [ ] Correlation preservation algorithms
- [ ] Quality validation pipeline
- [ ] Medical data-specific generators

### **Phase 3: Marketplace (Week 7-8)**
- [ ] Public dataset catalog
- [ ] ICP payment integration
- [ ] Revenue sharing system
- [ ] Download and licensing

### **Phase 4: Enterprise (Week 9-10)**
- [ ] API endpoints for programmatic access
- [ ] Compliance reporting tools
- [ ] Team workspace features
- [ ] Advanced privacy controls

---

## 🔲 REMAINING MENU IMPLEMENTATIONS

### **Currently Working Menu Items:**
- [x] **Upload Dataset** - Fully functional CSV upload with encryption
- [x] **My Datasets** - Browse and manage uploaded datasets
- [x] **Sample Data** - Pre-loaded synthetic datasets for testing

### **Menu Items Needing Implementation:**

#### **Main Section:**
- [ ] **Marketplace** - Public dataset catalog and purchase system
  - [ ] Dataset discovery and search interface
  - [ ] Dataset preview and metadata display
  - [ ] Purchase workflow with ICP payments
  - [ ] Download and access management

#### **Workspace Section:**
- [ ] **Generate Synthetic** - Core synthetic data generation
  - [ ] Data profiling and analysis dashboard
  - [ ] Privacy settings configuration (k-anonymity, differential privacy)
  - [ ] Synthetic data generation parameters
  - [ ] Quality metrics and validation results
  - [ ] Export synthetic datasets

- [ ] **Analytics** - Data insights and quality metrics
  - [ ] Statistical analysis of uploaded datasets
  - [ ] Data quality scoring and recommendations
  - [ ] Privacy risk assessment dashboard
  - [ ] Correlation and distribution analysis
  - [ ] Synthetic vs. original data comparison

- [ ] **Settings** - User preferences and configuration
  - [ ] Privacy preferences and default settings
  - [ ] Account management and profile
  - [ ] API key generation and management
  - [ ] Notification preferences
  - [ ] Data retention and deletion policies

### **Priority Implementation Order:**
1. **Generate Synthetic** (Core Gretel.ai feature)
2. **Marketplace** (Monetization and discovery)
3. **Analytics** (Data intelligence and insights)
4. **Settings** (User management and preferences)

**Target**: Transform from "encrypted notes app" → "full Gretel.ai competitor" with ICP's decentralization advantages and medical data focus.