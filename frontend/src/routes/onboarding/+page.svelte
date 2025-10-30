<script lang="ts">
	import Button from '@smui/button';
	import Card from '@smui/card';
	import Textfield from '@smui/textfield';
	import { goto } from '$app/navigation';
	import { updateUser } from '$lib/stores/user';

	let fullName = $state('');
	let profession = $state('');
	let licenseNumber = $state('');
	let selectedState = $state('');
	let renewalDate = $state('');
	let hoursRequired = $state('');
	let hoursCompleted = $state('');
	let customTopic = $state('');
	let selectedTopics = $state<string[]>([]);
	let suggestedTopics = $state<string[]>([]);

	// CE hours lookup by profession and state
	const ceRequirements: Record<string, Record<string, number>> = {
		'Attorney': {
			'CA': 25, 'NY': 24, 'TX': 15, 'FL': 33, 'IL': 30,
			'default': 24
		},
		'Engineer': {
			'CA': 30, 'TX': 15, 'NY': 36, 'FL': 18, 'IL': 30,
			'default': 24
		},
		'CPA': {
			'CA': 80, 'TX': 120, 'NY': 40, 'FL': 80, 'IL': 120,
			'default': 40
		},
		'Architect': {
			'CA': 30, 'TX': 12, 'NY': 36, 'FL': 20, 'IL': 24,
			'default': 24
		}
	};

	// AI-suggested topics by profession
	const topicSuggestions: Record<string, string[]> = {
		'Attorney': ['Contract Law', 'Ethics & Professional Responsibility', 'Litigation Strategy', 'Legal Technology', 'Employment Law', 'Intellectual Property', 'Tax Law', 'Real Estate Law'],
		'Engineer': ['Project Management', 'Sustainable Design', 'Building Codes', 'Safety Regulations', 'Advanced Materials', 'AutoCAD & BIM', 'Structural Analysis', 'Ethics'],
		'CPA': ['Tax Planning', 'Audit Standards', 'Financial Reporting', 'Ethics', 'Forensic Accounting', 'Technology & Data Analytics', 'Business Valuation', 'GAAP Updates'],
		'Architect': ['Building Codes', 'Sustainable Design', 'BIM & Technology', 'Project Management', 'Historic Preservation', 'Accessibility Standards', 'Construction Administration', 'Ethics']
	};

	// Update CE hours and suggested topics when profession or state changes
	$effect(() => {
		if (profession && selectedState) {
			const professionReqs = ceRequirements[profession];
			if (professionReqs) {
				const hours = professionReqs[selectedState] || professionReqs['default'];
				hoursRequired = hours.toString();
			}
		}

		if (profession && topicSuggestions[profession]) {
			suggestedTopics = topicSuggestions[profession];
		}
	});

	function toggleTopic(topic: string) {
		if (selectedTopics.includes(topic)) {
			selectedTopics = selectedTopics.filter(t => t !== topic);
		} else {
			selectedTopics = [...selectedTopics, topic];
		}
	}

	function addCustomTopic() {
		if (customTopic.trim() && !selectedTopics.includes(customTopic.trim())) {
			selectedTopics = [...selectedTopics, customTopic.trim()];
			customTopic = '';
		}
	}

	function handleSubmit(e: Event) {
		e.preventDefault();
		// Save user data to store
		updateUser({
			fullName,
			profession,
			state: selectedState,
			licenseNumber,
			renewalDate,
			hoursRequired: Number(hoursRequired),
			hoursCompleted: Number(hoursCompleted),
			selectedTopics
		});

		// Navigate to dashboard
		goto('/dashboard');
	}
</script>

<main class="onboarding-container" aria-label="Onboarding page">
	<section class="onboarding-content">
		<header class="header-section">
			<h1 class="page-title">Welcome to EduTrack Pro!</h1>
			<p class="subtitle">Let's personalize your continuing education experience</p>
		</header>

		<Card style="padding: 2.5rem; width: 100%; max-width: 700px;">
			<form onsubmit={handleSubmit} aria-label="Onboarding form">

				<!-- Section 1: Basic Information -->
				<section class="form-section">
					<h2 class="section-title">
						<span class="step-number" aria-label="Step 1">1</span>
						Basic Information
					</h2>

					<div class="form-row">
						<div class="form-field">
							<Textfield
								bind:value={fullName}
								label="Full Name"
								type="text"
								style="width: 100%;"
								variant="outlined"
								required
								input$aria-label="Your full name"
							/>
						</div>
					</div>

					<div class="form-row two-col">
						<div class="form-field">
							<label class="select-label" for="profession-select">Profession *</label>
							<select
								id="profession-select"
								bind:value={profession}
								required
								class="custom-select"
								aria-label="Select your profession"
							>
								<option value="">Select profession</option>
								<option value="Attorney">Attorney</option>
								<option value="Engineer">Professional Engineer</option>
								<option value="CPA">Certified Public Accountant</option>
								<option value="Architect">Architect</option>
							</select>
						</div>

						<div class="form-field">
							<label class="select-label" for="state-select">State/Province *</label>
							<select
								id="state-select"
								bind:value={selectedState}
								required
								class="custom-select"
								aria-label="Select your state or province"
							>
								<option value="">Select state</option>
								<option value="CA">California</option>
								<option value="NY">New York</option>
								<option value="TX">Texas</option>
								<option value="FL">Florida</option>
								<option value="IL">Illinois</option>
							</select>
						</div>
					</div>

					<div class="form-row two-col">
						<div class="form-field">
							<Textfield
								bind:value={licenseNumber}
								label="License Number"
								type="text"
								style="width: 100%;"
								variant="outlined"
								required
								input$aria-label="Your license number"
							/>
						</div>

						<div class="form-field">
							<Textfield
								bind:value={renewalDate}
								label="Renewal Date"
								type="date"
								style="width: 100%;"
								variant="outlined"
								required
								input$aria-label="License renewal date"
							/>
						</div>
					</div>
				</section>

				<div class="section-divider"></div>

				<!-- Section 2: CE Hours -->
				<section class="form-section">
					<h2 class="section-title">
						<span class="step-number" aria-label="Step 2">2</span>
						Continuing Education Status
					</h2>

					{#if hoursRequired}
						<div class="ai-badge">
							<span class="material-icons">auto_awesome</span>
							<span>AI auto-filled based on your profession and state</span>
						</div>
					{/if}

					<div class="form-row two-col">
						<div class="form-field">
							<Textfield
								bind:value={hoursRequired}
								label="Hours Required"
								type="number"
								style="width: 100%;"
								variant="outlined"
								required
								input$aria-label="Required CE hours"
							/>
						</div>

						<div class="form-field">
							<Textfield
								bind:value={hoursCompleted}
								label="Hours Completed"
								type="number"
								style="width: 100%;"
								variant="outlined"
								required
								input$aria-label="Completed CE hours"
							/>
						</div>
					</div>

					{#if hoursRequired && hoursCompleted}
						<div class="progress-preview">
							<div class="progress-info">
								<span class="progress-label">Your Progress</span>
								<span class="progress-numbers">{hoursCompleted} / {hoursRequired} hours</span>
							</div>
							<div class="progress-bar">
								<div
									class="progress-fill"
									style="width: {Math.min(100, (Number(hoursCompleted) / Number(hoursRequired)) * 100)}%"
									role="progressbar"
									aria-valuenow={Number(hoursCompleted)}
									aria-valuemin="0"
									aria-valuemax={Number(hoursRequired)}
									aria-label="CE hours completion progress"
								></div>
							</div>
						</div>
					{/if}
				</section>

				<div class="section-divider"></div>

				<!-- Section 3: Topic Interests -->
				<section class="form-section">
					<h2 class="section-title">
						<span class="step-number" aria-label="Step 3">3</span>
						Areas of Interest
					</h2>

					{#if suggestedTopics.length > 0}
						<div class="ai-badge">
							<span class="material-icons">auto_awesome</span>
							<span>AI-powered recommendations for {profession}s</span>
						</div>

						<p class="section-description">Select topics you're interested in to get personalized course recommendations</p>

						<div class="topic-chips" role="group" aria-label="Topic selection">
							{#each suggestedTopics as topic}
								<button
									type="button"
									class="topic-chip"
									class:selected={selectedTopics.includes(topic)}
									onclick={() => toggleTopic(topic)}
									aria-pressed={selectedTopics.includes(topic)}
									aria-label={`${topic} topic`}
								>
									{#if selectedTopics.includes(topic)}
										<span class="material-icons chip-icon">check_circle</span>
									{/if}
									{topic}
								</button>
							{/each}
						</div>

						<div class="custom-topic-section">
							<p class="custom-topic-label">Don't see what you're looking for?</p>
							<div class="custom-topic-input">
								<Textfield
									bind:value={customTopic}
									label="Add custom topic"
									type="text"
									style="flex: 1;"
									variant="outlined"
									input$aria-label="Add custom topic"
									on:keydown={(e) => {
										if (e.key === 'Enter') {
											e.preventDefault();
											addCustomTopic();
										}
									}}
								/>
								<Button
									variant="outlined"
									on:click={addCustomTopic}
									disabled={!customTopic.trim()}
									style="min-height: 44px;"
									aria-label="Add custom topic"
								>
									Add
								</Button>
							</div>
						</div>
					{/if}
				</section>

				<div class="info-box">
					<span class="material-icons" aria-hidden="true">info</span>
					<p>Your information is used to track requirements, suggest relevant courses, and automate certification submissions to your governing body.</p>
				</div>

				<div class="button-group">
					<Button
						variant="outlined"
						style="flex: 1; min-height: 44px;"
						on:click={() => goto('/')}
						aria-label="Go back to login"
					>
						Back
					</Button>
					<Button
						variant="raised"
						class="submit-button"
						type="submit"
						style="flex: 2; min-height: 44px;"
						aria-label="Complete onboarding and go to dashboard"
					>
						<span style="font-weight: 600;">Start Learning</span>
					</Button>
				</div>
			</form>
		</Card>
	</section>
</main>

<style>
	.onboarding-container {
		min-height: 100vh;
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
		padding: 2rem 1rem;
	}

	.onboarding-content {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 2rem;
		max-width: 800px;
		width: 100%;
	}

	.header-section {
		text-align: center;
		color: white;
	}

	.page-title {
		font-size: 2.5rem;
		font-weight: 700;
		margin-bottom: 0.5rem;
		background: linear-gradient(135deg, #fff 0%, #e0e7ff 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}

	.subtitle {
		font-size: 1.1rem;
		color: #f3f4f6;
		font-weight: 300;
	}

	.form-section {
		margin-bottom: 2rem;
	}

	.section-title {
		font-size: 1.25rem;
		font-weight: 600;
		color: #1f2937;
		margin-bottom: 1rem;
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.step-number {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		background: #3B82F6;
		color: white;
		border-radius: 50%;
		font-size: 1rem;
		font-weight: 700;
	}

	.section-divider {
		height: 1px;
		background: linear-gradient(to right, transparent, #e5e7eb, transparent);
		margin: 2rem 0;
	}

	.section-description {
		font-size: 0.9rem;
		color: #6b7280;
		margin-bottom: 1rem;
	}

	.form-row {
		margin-bottom: 1.5rem;
	}

	.two-col {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
	}

	.form-field {
		position: relative;
	}

	.select-label {
		display: block;
		font-size: 0.875rem;
		font-weight: 500;
		color: #374151;
		margin-bottom: 0.5rem;
	}

	.custom-select {
		width: 100%;
		padding: 0.875rem 1rem;
		font-size: 1rem;
		font-family: Inter, system-ui, sans-serif;
		border: 1px solid #d1d5db;
		border-radius: 4px;
		background: white;
		color: #1f2937;
		cursor: pointer;
		transition: all 0.2s;
	}

	.custom-select:hover {
		border-color: #9ca3af;
	}

	.custom-select:focus {
		outline: 2px solid #3B82F6;
		outline-offset: 2px;
		border-color: #3B82F6;
	}

	.ai-badge {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		background: linear-gradient(135deg, #eff6ff 0%, #f3e8ff 100%);
		border: 1px solid #c7d2fe;
		border-radius: 20px;
		font-size: 0.875rem;
		color: #4338ca;
		font-weight: 500;
		margin-bottom: 1rem;
	}

	.ai-badge .material-icons {
		font-size: 1.125rem;
		color: #7c3aed;
	}

	.progress-preview {
		margin-top: 1rem;
		padding: 1rem;
		background: #f9fafb;
		border-radius: 8px;
	}

	.progress-info {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.5rem;
	}

	.progress-label {
		font-size: 0.875rem;
		font-weight: 500;
		color: #374151;
	}

	.progress-numbers {
		font-size: 0.875rem;
		font-weight: 600;
		color: #1f2937;
	}

	.progress-bar {
		width: 100%;
		height: 8px;
		background: #e5e7eb;
		border-radius: 4px;
		overflow: hidden;
	}

	.progress-fill {
		height: 100%;
		background: linear-gradient(90deg, #3B82F6 0%, #A855F7 100%);
		transition: width 0.3s ease;
	}

	.topic-chips {
		display: flex;
		flex-wrap: wrap;
		gap: 0.75rem;
		margin-bottom: 1.5rem;
	}

	.topic-chip {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1.25rem;
		background: white;
		border: 2px solid #e5e7eb;
		border-radius: 24px;
		font-size: 0.875rem;
		font-weight: 500;
		color: #374151;
		cursor: pointer;
		transition: all 0.2s;
		min-height: 44px;
	}

	.topic-chip:hover {
		border-color: #9ca3af;
		background: #f9fafb;
	}

	.topic-chip:focus {
		outline: 2px solid #3B82F6;
		outline-offset: 2px;
	}

	.topic-chip.selected {
		background: linear-gradient(135deg, #3B82F6 0%, #A855F7 100%);
		border-color: transparent;
		color: white;
	}

	.chip-icon {
		font-size: 1.125rem;
	}

	.custom-topic-section {
		margin-top: 1.5rem;
	}

	.custom-topic-label {
		font-size: 0.875rem;
		color: #6b7280;
		margin-bottom: 0.75rem;
	}

	.custom-topic-input {
		display: flex;
		gap: 0.75rem;
		align-items: flex-start;
	}

	.info-box {
		display: flex;
		gap: 0.75rem;
		padding: 1rem;
		background: #eff6ff;
		border-left: 4px solid #3B82F6;
		border-radius: 4px;
		margin-bottom: 1.5rem;
	}

	.info-box .material-icons {
		color: #3B82F6;
		font-size: 1.25rem;
		flex-shrink: 0;
	}

	.info-box p {
		font-size: 0.875rem;
		color: #1e40af;
		margin: 0;
		line-height: 1.5;
	}

	.button-group {
		display: flex;
		gap: 1rem;
		margin-top: 1rem;
	}

	:global(.submit-button),
	:global(.submit-button::before),
	:global(.submit-button .mdc-button__ripple) {
		background: linear-gradient(135deg, #3B82F6 0%, #A855F7 100%) !important;
		color: white !important;
	}

	:global(.submit-button) {
		border-radius: 4px;
		overflow: hidden;
	}

	:global(.submit-button:focus),
	:global(.mdc-button:focus) {
		outline: 2px solid #1f2937;
		outline-offset: 2px;
	}

	:global(.mdc-text-field:focus-within) {
		outline: 2px solid #3B82F6;
		outline-offset: 2px;
		border-radius: 4px;
	}

	/* Mobile responsive styles */
	@media (max-width: 768px) {
		.onboarding-container {
			padding: 1rem 0.5rem;
		}

		.page-title {
			font-size: 1.75rem;
		}

		.subtitle {
			font-size: 0.95rem;
		}

		.two-col {
			grid-template-columns: 1fr;
			gap: 1.5rem;
		}

		:global(.mdc-card) {
			padding: 1.5rem !important;
		}

		.button-group {
			flex-direction: column;
		}

		.button-group :global(.mdc-button) {
			width: 100%;
		}

		.topic-chips {
			gap: 0.5rem;
		}

		.topic-chip {
			font-size: 0.8rem;
			padding: 0.5rem 1rem;
		}

		.custom-topic-input {
			flex-direction: column;
		}

		.custom-topic-input :global(.mdc-button) {
			width: 100%;
		}
	}
</style>
