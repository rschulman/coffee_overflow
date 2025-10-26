<script lang="ts">
	import Card from '@smui/card';
	import Textfield from '@smui/textfield';
	import Button from '@smui/button';
	import { goto } from '$app/navigation';
	import { updateUser } from '$lib/stores/user';
	import { CE_REQUIREMENTS, TOPIC_SUGGESTIONS } from '$lib/data/constants';
	import { STRINGS, formatString } from '$lib/constants/strings';
	import GradientButton from '$lib/components/shared/GradientButton.svelte';
	import FormSection from '$lib/components/onboarding/FormSection.svelte';
	import TopicChip from '$lib/components/onboarding/TopicChip.svelte';
	import ProgressPreview from '$lib/components/onboarding/ProgressPreview.svelte';

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

	// Update CE hours and suggested topics when profession or state changes
	$effect(() => {
		if (profession && selectedState) {
			const professionReqs = CE_REQUIREMENTS[profession];
			if (professionReqs) {
				const hours = professionReqs[selectedState] || professionReqs['default'];
				hoursRequired = hours.toString();
			}
		}

		if (profession && TOPIC_SUGGESTIONS[profession]) {
			suggestedTopics = TOPIC_SUGGESTIONS[profession];
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

	function handleSubmit() {
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
		goto('/dashboard');
	}
</script>

<main class="onboarding-container" aria-label={STRINGS.aria.onboardingPage}>
	<nav class="top-nav" aria-label={STRINGS.aria.mainNav}>
		<a href="/login" class="login-link" aria-label={STRINGS.aria.alreadyHaveAccountSignIn}>
			{STRINGS.nav.alreadyHaveAccount} <strong>{STRINGS.nav.signIn}</strong>
		</a>
	</nav>

	<section class="onboarding-content">
		<header class="header-section">
			<h1 class="page-title">{STRINGS.onboarding.title}</h1>
			<p class="subtitle">{STRINGS.onboarding.subtitle}</p>
		</header>

		<Card style="padding: 2.5rem; width: 100%; max-width: 700px;">
			<form on:submit|preventDefault={handleSubmit} aria-label={STRINGS.aria.onboardingForm}>

				<!-- Section 1: Basic Information -->
				<FormSection stepNumber={1} title={STRINGS.onboarding.section1Title}>
					<div class="form-row">
						<div class="form-field">
							<Textfield
								bind:value={fullName}
								label={STRINGS.onboarding.fullName}
								type="text"
								style="width: 100%;"
								variant="outlined"
								required
								input$aria-label={STRINGS.aria.yourFullName}
							/>
						</div>
					</div>

					<div class="form-row two-col">
						<div class="form-field">
							<label class="select-label" for="profession-select">{STRINGS.onboarding.profession} *</label>
							<select
								id="profession-select"
								bind:value={profession}
								required
								class="custom-select"
								aria-label={STRINGS.aria.selectYourProfession}
							>
								<option value="">{STRINGS.onboarding.selectProfession}</option>
								<option value="Attorney">{STRINGS.professions.attorney}</option>
								<option value="Engineer">{STRINGS.professions.engineer}</option>
								<option value="CPA">{STRINGS.professions.cpa}</option>
								<option value="Architect">{STRINGS.professions.architect}</option>
							</select>
						</div>

						<div class="form-field">
							<label class="select-label" for="state-select">{STRINGS.onboarding.state} *</label>
							<select
								id="state-select"
								bind:value={selectedState}
								required
								class="custom-select"
								aria-label={STRINGS.aria.selectYourState}
							>
								<option value="">{STRINGS.onboarding.selectState}</option>
								<option value="CA">{STRINGS.states.ca}</option>
								<option value="NY">{STRINGS.states.ny}</option>
								<option value="TX">{STRINGS.states.tx}</option>
								<option value="FL">{STRINGS.states.fl}</option>
								<option value="IL">{STRINGS.states.il}</option>
							</select>
						</div>
					</div>

					<div class="form-row two-col">
						<div class="form-field">
							<Textfield
								bind:value={licenseNumber}
								label={STRINGS.onboarding.licenseNumber}
								type="text"
								style="width: 100%;"
								variant="outlined"
								required
								input$aria-label={STRINGS.aria.yourLicenseNumber}
							/>
						</div>

						<div class="form-field">
							<Textfield
								bind:value={renewalDate}
								label={STRINGS.onboarding.renewalDate}
								type="date"
								style="width: 100%;"
								variant="outlined"
								required
								input$aria-label={STRINGS.aria.licenseRenewalDate}
							/>
						</div>
					</div>
				</FormSection>

				<div class="section-divider"></div>

				<!-- Section 2: CE Hours -->
				<FormSection stepNumber={2} title={STRINGS.onboarding.section2Title}>
					{#if hoursRequired}
						<div class="ai-badge">
							<span class="material-icons">auto_awesome</span>
							<span>{STRINGS.onboarding.aiAutoFilled}</span>
						</div>
					{/if}

					<div class="form-row two-col">
						<div class="form-field">
							<Textfield
								bind:value={hoursRequired}
								label={STRINGS.onboarding.hoursRequired}
								type="number"
								style="width: 100%;"
								variant="outlined"
								required
								input$aria-label={STRINGS.aria.requiredCEHours}
							/>
						</div>

						<div class="form-field">
							<Textfield
								bind:value={hoursCompleted}
								label={STRINGS.onboarding.hoursCompleted}
								type="number"
								style="width: 100%;"
								variant="outlined"
								required
								input$aria-label={STRINGS.aria.completedCEHours}
							/>
						</div>
					</div>

					{#if hoursRequired && hoursCompleted}
						<ProgressPreview
							hoursCompleted={Number(hoursCompleted)}
							hoursRequired={Number(hoursRequired)}
						/>
					{/if}
				</FormSection>

				<div class="section-divider"></div>

				<!-- Section 3: Topic Interests -->
				<FormSection stepNumber={3} title={STRINGS.onboarding.section3Title}>
					{#if suggestedTopics.length > 0}
						<div class="ai-badge">
							<span class="material-icons">auto_awesome</span>
							<span>{formatString(STRINGS.onboarding.aiRecommendations, { profession })}</span>
						</div>

						<p class="section-description">{STRINGS.onboarding.selectTopicsPrompt}</p>

						<div class="topic-chips" role="group" aria-label={STRINGS.aria.topicSelection}>
							{#each suggestedTopics as topic}
								<TopicChip
									{topic}
									selected={selectedTopics.includes(topic)}
									onToggle={toggleTopic}
								/>
							{/each}
						</div>

						<div class="custom-topic-section">
							<p class="custom-topic-label">{STRINGS.onboarding.dontSeeTopics}</p>
							<div class="custom-topic-input">
								<Textfield
									bind:value={customTopic}
									label={STRINGS.onboarding.addCustomTopic}
									type="text"
									style="flex: 1;"
									variant="outlined"
									input$aria-label={STRINGS.aria.addCustomTopicField}
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
									style="min-height: 56px; min-width: 100px; padding: 0 2rem;"
									aria-label={STRINGS.aria.addCustomTopicField}
								>
									<span style="font-weight: 600;">{STRINGS.onboarding.add}</span>
								</Button>
							</div>
						</div>
					{/if}
				</FormSection>

				<div class="info-box">
					<span class="material-icons" aria-hidden="true">info</span>
					<p>{STRINGS.onboarding.infoText}</p>
				</div>

				<div class="button-group">
					<GradientButton type="submit" fullWidth={true} ariaLabel={STRINGS.aria.completeOnboarding}>
						<span style="font-weight: 600;">{STRINGS.onboarding.startLearning}</span>
					</GradientButton>
				</div>
			</form>
		</Card>
	</section>
</main>

<style>
	.onboarding-container {
		min-height: 100vh;
		display: flex;
		flex-direction: column;
		background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
		padding: 0;
	}

	.top-nav {
		padding: 1.5rem 2rem;
		display: flex;
		justify-content: flex-end;
		width: 100%;
	}

	.login-link {
		color: white;
		text-decoration: none;
		font-size: 0.95rem;
		padding: 0.75rem 1.5rem;
		border-radius: 8px;
		background: rgba(255, 255, 255, 0.1);
		backdrop-filter: blur(10px);
		transition: all 0.2s;
		min-height: 44px;
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
	}

	.login-link:hover {
		background: rgba(255, 255, 255, 0.2);
	}

	.login-link:focus {
		outline: 2px solid white;
		outline-offset: 2px;
	}

	.login-link strong {
		font-weight: 600;
	}

	.onboarding-content {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 2rem;
		max-width: 800px;
		width: 100%;
		margin: 0 auto;
		padding: 2rem 1rem;
		flex: 1;
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

	.topic-chips {
		display: flex;
		flex-wrap: wrap;
		gap: 0.75rem;
		margin-bottom: 1.5rem;
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
		align-items: center;
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

	:global(.mdc-text-field) {
		width: 100% !important;
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

		.custom-topic-input {
			flex-direction: column;
		}

		.custom-topic-input :global(.mdc-button) {
			width: 100%;
		}
	}
</style>
