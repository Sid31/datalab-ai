<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { auth } from '../store/auth';
  import { get } from 'svelte/store';
  import { showError, showSuccess } from '../store/notifications';

  const dispatch = createEventDispatcher();

  let isLoading = false;

  // Sample datasets with different types of synthetic data
  const sampleDatasets = [
    {
      name: 'Customer Demographics',
      description: 'Synthetic customer data with demographics and purchasing behavior',
      headers: ['customer_id', 'name', 'email', 'age', 'city', 'income', 'segment'],
      rowCount: 1000,
      data: `customer_id,name,email,age,city,income,segment
1001,Alice Johnson,alice.j@email.com,32,Seattle,75000,Premium
1002,Bob Smith,bob.smith@email.com,28,Portland,52000,Standard
1003,Carol Davis,carol.d@email.com,45,San Francisco,95000,Premium
1004,David Wilson,david.w@email.com,35,Los Angeles,68000,Standard
1005,Emma Brown,emma.b@email.com,29,Denver,71000,Standard`
    },
    {
      name: 'Financial Transactions',
      description: 'Synthetic financial transaction data with privacy masking',
      headers: ['transaction_id', 'account_id', 'amount', 'category', 'date', 'merchant'],
      rowCount: 2500,
      data: `transaction_id,account_id,amount,category,date,merchant
TXN001,ACC****1234,125.50,Groceries,2024-01-15,SafeWay Store
TXN002,ACC****5678,45.00,Gas,2024-01-15,Shell Station
TXN003,ACC****9012,89.99,Electronics,2024-01-16,Best Buy
TXN004,ACC****3456,12.75,Coffee,2024-01-16,Starbucks
TXN005,ACC****7890,250.00,Utilities,2024-01-17,City Power`
    },
    {
      name: 'Healthcare Records',
      description: 'De-identified patient data for research purposes',
      headers: ['patient_id', 'age_group', 'condition', 'treatment', 'outcome', 'region'],
      rowCount: 750,
      data: `patient_id,age_group,condition,treatment,outcome,region
P****001,25-35,Hypertension,Medication A,Improved,North
P****002,45-55,Diabetes,Insulin,Stable,South
P****003,35-45,Asthma,Inhaler,Improved,East
P****004,55-65,Arthritis,Physical Therapy,Stable,West
P****005,25-35,Migraine,Medication B,Improved,North`
    },
    {
      name: 'Medical Synthetic Demo',
      description: 'Optimized medical dataset for synthetic data generation testing',
      headers: ['id', 'age', 'gender', 'diagnosis', 'treatment', 'outcome', 'date'],
      rowCount: 100,
      data: `id,age,gender,diagnosis,treatment,outcome,date
MED001,34,F,ICD_1050,TX_150,Improved,2024-01-15
MED002,67,M,ICD_1120,TX_200,Stable,2024-01-16
MED003,45,F,ICD_1075,TX_175,Improved,2024-01-17
MED004,29,M,ICD_1200,TX_125,Stable,2024-01-18
MED005,52,F,ICD_1300,TX_250,Improved,2024-01-19
MED006,38,M,ICD_1150,TX_180,Stable,2024-01-20
MED007,61,F,ICD_1400,TX_220,Improved,2024-01-21
MED008,43,M,ICD_1250,TX_160,Stable,2024-01-22
MED009,36,F,ICD_1350,TX_190,Improved,2024-01-23
MED010,55,M,ICD_1180,TX_210,Stable,2024-01-24`
    },
    {
      name: 'IoT Sensor Data',
      description: 'Synthetic sensor readings from smart devices',
      headers: ['device_id', 'timestamp', 'temperature', 'humidity', 'location', 'status'],
      rowCount: 5000,
      data: `device_id,timestamp,temperature,humidity,location,status
DEV001,2024-01-15T08:00:00Z,22.5,45,Building A Floor 1,Normal
DEV002,2024-01-15T08:00:00Z,21.8,48,Building A Floor 2,Normal
DEV003,2024-01-15T08:00:00Z,23.1,42,Building B Floor 1,Normal
DEV004,2024-01-15T08:00:00Z,20.9,51,Building B Floor 2,Alert
DEV005,2024-01-15T08:00:00Z,22.3,46,Building C Floor 1,Normal`
    }
  ];

  async function loadSampleDataset(dataset: any) {
    isLoading = true;
    try {
      const $auth = get(auth);
      if (!$auth || $auth.state !== 'initialized') {
        showError('Authentication required');
        return;
      }

      // Create empty note first, then update with data
      const noteId = await $auth.actor.create_note();
      
      // Update the note with dataset JSON
      const datasetInfo = JSON.stringify({
        fileName: `${dataset.name.toLowerCase().replace(/\s+/g, '_')}.csv`,
        type: 'synthetic_dataset',
        headers: dataset.headers,
        content: dataset.data,
        rowCount: dataset.rowCount,
        uploadedAt: new Date().toISOString(),
        sampleDataset: true,
        description: dataset.description
      });
      
      await $auth.actor.update_note(noteId, datasetInfo);

      console.log('Sample dataset loaded successfully:', noteId);
      showSuccess(`Sample dataset "${dataset.name}" loaded successfully`);
      dispatch('loaded', { 
        datasetId: noteId, 
        fileName: `${dataset.name.toLowerCase().replace(/\s+/g, '_')}.csv`, 
        headers: dataset.headers, 
        rowCount: dataset.rowCount 
      });
      
    } catch (error) {
      showError(`Failed to load sample dataset: ${error.message}`);
    } finally {
      isLoading = false;
    }
  }
</script>

<div class="card bg-base-100 shadow-xl">
  <div class="card-body">
    <h2 class="card-title mb-4">Load Sample Datasets</h2>
    <p class="text-base-content/70 mb-6">
      Choose from pre-configured synthetic datasets to explore the platform features
    </p>
    
    <div class="grid gap-4">
      {#each sampleDatasets as dataset}
        <div class="card bg-base-200 shadow-sm">
          <div class="card-body p-4">
            <div class="flex justify-between items-start">
              <div class="flex-1">
                <h3 class="font-semibold text-lg">{dataset.name}</h3>
                <p class="text-sm text-base-content/70 mb-3">{dataset.description}</p>
                
                <div class="flex flex-wrap gap-2 mb-3">
                  <div class="badge badge-outline badge-sm">
                    {dataset.rowCount.toLocaleString()} rows
                  </div>
                  <div class="badge badge-outline badge-sm">
                    {dataset.headers.length} columns
                  </div>
                </div>
                
                <div class="text-xs text-base-content/60">
                  <strong>Columns:</strong> {dataset.headers.join(', ')}
                </div>
              </div>
              
              <button 
                class="btn btn-primary btn-sm ml-4"
                on:click={() => loadSampleDataset(dataset)}
                disabled={isLoading}
              >
                {#if isLoading}
                  <span class="loading loading-spinner loading-xs"></span>
                {:else}
                  Load Dataset
                {/if}
              </button>
            </div>
          </div>
        </div>
      {/each}
    </div>

    <div class="alert alert-info mt-6">
      <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-current shrink-0 w-6 h-6">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
      </svg>
      <div>
        <h3 class="font-bold">About Sample Data</h3>
        <div class="text-sm">
          All sample datasets contain synthetic data generated for demonstration purposes. 
          Sensitive fields are pre-masked for privacy compliance.
        </div>
      </div>
    </div>
  </div>
</div>
