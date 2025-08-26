<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { auth } from '../store/auth';
  import { get } from 'svelte/store';
  import { showError, showSuccess } from '../store/notifications';

  const dispatch = createEventDispatcher();

  let fileName = '';
  let fileContent = '';
  let isUploading = false;
  let dragOver = false;
  let fileInput: HTMLInputElement;

  // Sample CSV data for demonstration
  const sampleCSV = `name,email,age,city,salary
John Doe,john.doe@email.com,28,New York,75000
Jane Smith,jane.smith@email.com,32,Los Angeles,82000
Mike Johnson,mike.johnson@email.com,25,Chicago,68000
Sarah Wilson,sarah.wilson@email.com,29,Houston,71000
David Brown,david.brown@email.com,35,Phoenix,85000`;

  function handleFileSelect(event: Event) {
    const target = event.target as HTMLInputElement;
    const file = target.files?.[0];
    if (file) {
      processFile(file);
    }
  }

  function handleDrop(event: DragEvent) {
    event.preventDefault();
    dragOver = false;
    
    const file = event.dataTransfer?.files[0];
    if (file && file.type === 'text/csv') {
      processFile(file);
    } else {
      showError('Please upload a CSV file');
    }
  }

  function handleDragOver(event: DragEvent) {
    event.preventDefault();
    dragOver = true;
  }

  function handleDragLeave() {
    dragOver = false;
  }

  async function processFile(file: File) {
    fileName = file.name;
    try {
      fileContent = await file.text();
      showSuccess(`File "${fileName}" loaded successfully`);
    } catch (error) {
      showError('Failed to read file');
    }
  }

  function loadSampleData() {
    fileName = 'sample_employees.csv';
    fileContent = sampleCSV;
    showSuccess('Sample data loaded');
  }

  async function uploadDataset() {
    if (!fileContent) {
      showError('Please select a CSV file first');
      return;
    }

    isUploading = true;
    try {
      const $auth = get(auth);
      if (!$auth || $auth.state !== 'initialized') {
        showError('Authentication required');
        return;
      }

      // Parse CSV headers for metadata
      const lines = fileContent.split('\n');
      const headers = lines[0].split(',').map(h => h.trim());
      const dataRows = lines.slice(1).filter(line => line.trim());

      // Create empty note first, then update with data
      const noteId = await $auth.actor.create_note();
      
      // Update the note with dataset JSON
      const datasetInfo = JSON.stringify({
        fileName: fileName,
        type: 'csv_dataset',
        headers: headers,
        content: fileContent,
        rowCount: dataRows.length,
        uploadedAt: new Date().toISOString()
      });
      
      console.log('🔍 About to update note:', noteId, 'with data:', datasetInfo);
      await $auth.actor.update_note(noteId, datasetInfo);
      console.log('✅ Note updated successfully');

      console.log('Dataset uploaded successfully:', noteId);
      showSuccess(`Dataset uploaded successfully`);
      dispatch('uploaded', { datasetId: noteId, fileName, headers, rowCount: dataRows.length });
      
      // Reset form
      fileName = '';
      fileContent = '';
      if (fileInput) fileInput.value = '';
      
    } catch (error) {
      showError(`Failed to upload dataset: ${error.message}`);
    } finally {
      isUploading = false;
    }
  }
</script>

<div class="card bg-base-100 shadow-xl">
  <div class="card-body">
    <h2 class="card-title mb-4">Upload CSV Dataset</h2>
    
    <!-- File Drop Zone -->
    <div 
      class="border-2 border-dashed rounded-lg p-8 text-center transition-colors {dragOver ? 'border-primary bg-primary/10' : 'border-base-300'}"
      on:drop={handleDrop}
      on:dragover={handleDragOver}
      on:dragleave={handleDragLeave}
    >
      {#if fileName}
        <div class="text-success mb-4">
          <svg class="w-12 h-12 mx-auto mb-2" fill="currentColor" viewBox="0 0 20 20">
            <path d="M9 2a1 1 0 000 2h2a1 1 0 100-2H9z"/>
            <path fill-rule="evenodd" d="M4 5a2 2 0 012-2v1a1 1 0 001 1h6a1 1 0 001-1V3a2 2 0 012 2v6.5a1.5 1.5 0 01-1.5 1.5h-7A1.5 1.5 0 014 11.5V5zm8 5a1 1 0 10-2 0v1.5a.5.5 0 01-.5.5h-3a.5.5 0 01-.5-.5V10a1 1 0 10-2 0v1.5A2.5 2.5 0 006.5 14h3a2.5 2.5 0 002.5-2.5V10z" clip-rule="evenodd"/>
          </svg>
          <p class="font-semibold">{fileName}</p>
          <p class="text-sm opacity-70">{fileContent.split('\n').length - 1} rows detected</p>
        </div>
      {:else}
        <div class="text-base-content/70">
          <svg class="w-12 h-12 mx-auto mb-4" stroke="currentColor" fill="none" viewBox="0 0 48 48">
            <path d="M28 8H12a4 4 0 00-4 4v20m32-12v8m0 0v8a4 4 0 01-4 4H12a4 4 0 01-4-4v-4m32-4l-3.172-3.172a4 4 0 00-5.656 0L28 28M8 32l9.172-9.172a4 4 0 015.656 0L28 28m0 0l4 4m4-24h8m-4-4v8m-12 4h.02" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
          <p class="text-lg font-semibold mb-2">Drop your CSV file here</p>
          <p class="text-sm mb-4">or click to browse</p>
        </div>
      {/if}
      
      <input 
        type="file" 
        accept=".csv" 
        class="hidden" 
        bind:this={fileInput}
        on:change={handleFileSelect}
      />
      
      <button 
        class="btn btn-outline btn-sm"
        on:click={() => fileInput?.click()}
        disabled={isUploading}
      >
        Browse Files
      </button>
    </div>

    <!-- Sample Data Button -->
    <div class="divider">OR</div>
    
    <div class="text-center">
      <button 
        class="btn btn-secondary btn-outline"
        on:click={loadSampleData}
        disabled={isUploading}
      >
        Load Sample Employee Data
      </button>
      <p class="text-xs text-base-content/60 mt-2">
        Loads a sample CSV with employee information for testing
      </p>
    </div>

    <!-- File Preview -->
    {#if fileContent}
      <div class="mt-6">
        <h3 class="font-semibold mb-2">Preview</h3>
        <div class="bg-base-200 p-4 rounded-lg max-h-48 overflow-auto">
          <pre class="text-xs">{fileContent.split('\n').slice(0, 6).join('\n')}{fileContent.split('\n').length > 6 ? '\n...' : ''}</pre>
        </div>
      </div>
    {/if}

    <!-- Upload Button -->
    <div class="card-actions justify-end mt-6">
      <button 
        class="btn btn-primary"
        on:click={uploadDataset}
        disabled={!fileContent || isUploading}
      >
        {#if isUploading}
          <span class="loading loading-spinner loading-sm"></span>
          Uploading...
        {:else}
          Upload Dataset
        {/if}
      </button>
    </div>
  </div>
</div>
