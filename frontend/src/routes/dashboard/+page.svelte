<script lang="ts">
	import Card from '@smui/card';
	import Textfield from '@smui/textfield';
	import Button from '@smui/button';
	import { user, updateUser } from '$lib/stores/user';
	import { getRecommendations } from '$lib/network/api';
	import { STRINGS, formatString } from '$lib/constants/strings';
	import type { Course } from '$lib/types';
	import AppBar from '$lib/components/dashboard/AppBar.svelte';
	import StatCard from '$lib/components/dashboard/StatCard.svelte';
	import ProgressCircle from '$lib/components/dashboard/ProgressCircle.svelte';
	import CourseCard from '$lib/components/dashboard/CourseCard.svelte';
	import GradientButton from '$lib/components/shared/GradientButton.svelte';
	import { updateHours, getUserDetails } from '$lib/network/api';

	// Get user data from store
	const userData = $derived($user);
	const userName = $derived(userData.fullName.split(' ')[0] || 'User');

	// Calculate totals from all states
	const totalHoursCompleted = $derived(
		userData.stateHours.reduce((sum, sh) => sum + sh.hoursCompleted, 0)
	);

	// Sum up total hours required across all states
	const totalHoursRequired = $derived(
		userData.stateHours.reduce((sum, sh) => sum + sh.hoursRequired, 0)
	);

	// Get primary state (first one added, or empty)
	const primaryState = $derived(userData.stateHours[0]?.state || '');

	// Find the earliest renewal date across all states (most urgent)
	const nextRenewal = $derived.by(() => {
		const validDates = userData.stateHours
			.filter(sh => sh.renewalDate)
			.map(sh => ({
				state: sh.state,
				date: sh.renewalDate
			}))
			.sort((a, b) => new Date(a.date).getTime() - new Date(b.date).getTime());

		return validDates[0] || { state: '', date: '' };
	});

	const progressPercentage = $derived(
		totalHoursRequired > 0 ? (totalHoursCompleted / totalHoursRequired) * 100 : 0
	);
	const daysUntilRenewal = $derived(
		nextRenewal.date ? Math.ceil((new Date(nextRenewal.date).getTime() - new Date().getTime()) / (1000 * 60 * 60 * 24)) : 0
	);

	// AI-powered course recommendations
	let recommendedCourses = $state<Course[]>([]);
	let loadingRecommendations = $state(false);
	let userInterests = $state('');
	let recommendationsError = $state('');

	async function fetchRecommendations() {
		if (!userInterests.trim()) {
			recommendationsError = 'Please enter your interests';
			return;
		}

		loadingRecommendations = true;
		recommendationsError = '';

		try {
			const response = await getRecommendations(userInterests);
			recommendedCourses = response.recommendations.map((rec, idx) => ({
				id: idx + 1,
				...rec,
				aiReason: rec.ai_reason
			}));
		} catch (error) {
			console.error('Failed to get recommendations:', error);
			recommendationsError = 'Failed to get recommendations. Please try again.';
		} finally {
			loadingRecommendations = false;
		}
	}

	function formatDate(dateString: string) {
		return new Date(dateString).toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' });
	}

	function exportToCSV() {
		// Create CSV header
		const header = 'State,Hours Completed,Hours Required,Renewal Date\n';

		// Create CSV rows
		const rows = userData.stateHours.map(sh =>
			`${sh.state},${sh.hoursCompleted},${sh.hoursRequired},${sh.renewalDate || 'Not set'}`
		).join('\n');

		// Combine header and rows
		const csv = header + rows;

		// Create blob and download
		const blob = new Blob([csv], { type: 'text/csv;charset=utf-8;' });
		const link = document.createElement('a');
		const url = URL.createObjectURL(blob);
		link.setAttribute('href', url);
		link.setAttribute('download', `ce-hours-status-${new Date().toISOString().split('T')[0]}.csv`);
		link.style.visibility = 'hidden';
		document.body.appendChild(link);
		link.click();
		document.body.removeChild(link);
	}

	// State for editing hours
	let editingStateCode = $state<string | null>(null);
	let editHours = $state('');
	let updating = $state(false);

	function startEditing(stateCode: string, currentHours: number) {
		editingStateCode = stateCode;
		editHours = currentHours.toString();
	}

	function cancelEditing() {
		editingStateCode = null;
		editHours = '';
	}

	async function saveHours(stateCode: string) {
		if (!editHours || updating) return;

		updating = true;
		try {
			await updateHours({
				state_id: stateCode,
				hours: parseInt(editHours)
			});

			// Refresh user data from backend
			const userDetails = await getUserDetails();
			const stateHours = userDetails.states.map(state => ({
				state: state.state_code,
				hoursCompleted: state.hours_complete,
				hoursRequired: state.legal_hours,
				renewalDate: state.renewal_date || ''
			}));

			updateUser({
				username: userDetails.username,
				fullName: userDetails.fullname,
				stateHours
			});

			// Close editing
			editingStateCode = null;
			editHours = '';
		} catch (error) {
			console.error('Failed to update hours:', error);
			alert('Failed to update hours. Please try again.');
		} finally {
			updating = false;
		}
	}
</script>

<main class="dashboard-container">
	<AppBar />

	<!-- Main Content -->
	<div class="content-wrapper">
		<section class="dashboard-content">
			<!-- Welcome Section -->
			<div class="welcome-section">
				<div>
					<h2 class="welcome-title">{formatString(STRINGS.dashboard.welcomeBack, { name: userName })}</h2>
					<p class="welcome-subtitle">
						{#if primaryState}
							Licensed in {primaryState}
							{#if userData.stateHours.length > 1}
								+ {userData.stateHours.length - 1} more {userData.stateHours.length === 2 ? 'state' : 'states'}
							{/if}
						{:else}
							No state licenses added
						{/if}
					</p>
				</div>
				<GradientButton
					onclick={exportToCSV}
					ariaLabel="Export status to CSV"
				>
					<div class="button-content">
						<span class="material-icons button-icon">download</span>
						<span>Export CSV</span>
					</div>
				</GradientButton>
			</div>

			<!-- Stats Grid -->
			<div class="stats-grid">
				<!-- Progress Card -->
				<Card class="stat-card progress-card">
					<div class="stat-header">
						<div>
							<h3 class="stat-title">{STRINGS.dashboard.ceHoursProgress}</h3>
							<p class="stat-subtitle">{STRINGS.dashboard.currentPeriod}</p>
						</div>
						<div class="stat-icon progress-icon">
							<span class="material-icons">timeline</span>
						</div>
					</div>
					<ProgressCircle hoursCompleted={totalHoursCompleted} hoursRequired={totalHoursRequired} />
				</Card>

				<!-- Deadline Card -->
				<StatCard
					title={STRINGS.dashboard.nextRenewal}
					subtitle={nextRenewal.state ? `${nextRenewal.state} License` : STRINGS.dashboard.licenseExpiration}
					icon="event"
					iconBgColor="linear-gradient(135deg, #fef3c7 0%, #fde68a 100%)"
				>
					<p class="deadline-date">{nextRenewal.date ? formatDate(nextRenewal.date) : STRINGS.dashboard.notSet}</p>
					<p class="deadline-days">{formatString(STRINGS.dashboard.daysRemaining, { days: daysUntilRenewal })}</p>
				</StatCard>

				<!-- Completion Rate Card -->
				<StatCard
					title={STRINGS.dashboard.onTrack}
					subtitle={STRINGS.dashboard.completionStatus}
					icon="check_circle"
					iconBgColor="linear-gradient(135deg, #d1fae5 0%, #a7f3d0 100%)"
				>
					<p class="status-text">
						{#if progressPercentage >= 90}
							{STRINGS.dashboard.almostThere}
						{:else if progressPercentage >= 50}
							{STRINGS.dashboard.greatProgress}
						{:else}
							{STRINGS.dashboard.keepGoing}
						{/if}
					</p>
					<p class="status-subtitle">{STRINGS.dashboard.doingGreat}</p>
				</StatCard>
			</div>

			<!-- License Status by State Section -->
			<section class="license-status-section">
				<h3 class="section-title">License Status by State</h3>
				<Card style="padding: 0; overflow: hidden;">
					<div class="state-list">
						{#if userData.stateHours.length > 0}
							{#each userData.stateHours as stateHour}
								<div class="state-item">
									<div class="state-icon-wrapper">
										<span class="material-icons state-icon">location_on</span>
									</div>
									<div class="state-details">
										<h4 class="state-title">{stateHour.state}</h4>
										{#if editingStateCode === stateHour.state}
											<div class="edit-hours-form">
												<input
													type="number"
													bind:value={editHours}
													min={stateHour.hoursCompleted}
													max={stateHour.hoursRequired}
													class="hours-input"
													placeholder="Hours"
												/>
												<button
													onclick={() => saveHours(stateHour.state)}
													disabled={updating}
													class="save-btn"
													aria-label="Save hours"
												>
													<span class="material-icons">check</span>
												</button>
												<button
													onclick={cancelEditing}
													disabled={updating}
													class="cancel-btn"
													aria-label="Cancel"
												>
													<span class="material-icons">close</span>
												</button>
											</div>
										{:else}
											<p class="state-meta">
												{stateHour.hoursCompleted} / {stateHour.hoursRequired} hours completed
												{#if stateHour.renewalDate}
													• Renewal: {formatDate(stateHour.renewalDate)}
												{/if}
											</p>
										{/if}
									</div>
									<div class="state-status">
										{#if editingStateCode === stateHour.state}
											<span class="hours-badge editing">Editing...</span>
										{:else}
											<span class="hours-badge">{stateHour.hoursCompleted}/{stateHour.hoursRequired}</span>
											<button
												onclick={() => startEditing(stateHour.state, stateHour.hoursCompleted)}
												class="edit-btn"
												aria-label="Edit hours for {stateHour.state}"
											>
												<span class="material-icons">edit</span>
											</button>
										{/if}
									</div>
								</div>
							{/each}
						{:else}
							<div class="empty-state">
								<span class="material-icons empty-icon">info</span>
								<p>No state licenses added yet</p>
							</div>
						{/if}
					</div>
				</Card>
			</section>

			<!-- AI Recommendations Section -->
			<section class="recommendations-section">
				<div class="section-header">
					<div>
						<h3 class="section-title">
							<span class="material-icons ai-icon">auto_awesome</span>
							{STRINGS.dashboard.aiRecommendations}
						</h3>
						<p class="section-subtitle">
							Tell us what you're interested in learning
						</p>
					</div>
				</div>

				<!-- Interests Input -->
				<Card style="padding: 1.5rem; margin-bottom: 1.5rem;">
					<div class="interests-form">
						<Textfield
							bind:value={userInterests}
							label="What topics interest you?"
							style="width: 100%;"
							variant="outlined"
							placeholder="e.g., contract law, legal ethics, technology..."
							input$maxlength="200"
						/>
						<Button
							variant="raised"
							onclick={fetchRecommendations}
							disabled={loadingRecommendations || !userInterests.trim()}
							style="background: linear-gradient(135deg, #3B82F6 0%, #A855F7 100%); color: white; min-height: 44px;"
						>
							{#if loadingRecommendations}
								<span>Getting Recommendations...</span>
							{:else}
								<span>Get AI Recommendations</span>
							{/if}
						</Button>
					</div>
					{#if recommendationsError}
						<p class="error-message" role="alert" style="margin-top: 1rem; color: #dc2626;">{recommendationsError}</p>
					{/if}
				</Card>

				<div class="courses-grid">
					{#if loadingRecommendations}
						<p style="color: #6b7280;">Loading recommendations...</p>
					{:else if recommendedCourses.length === 0}
						<p style="color: #6b7280;">Enter your interests above to get personalized course recommendations</p>
					{:else}
						{#each recommendedCourses as course}
							<CourseCard
								title={course.title}
								provider={course.provider}
								topic={course.topic}
								hours={course.hours}
								format={course.format}
								price={course.price}
								url={course.url}
								rating={course.rating}
								aiReason={course.aiReason}
							/>
						{/each}
					{/if}
				</div>
			</section>
		</section>
	</div>
</main>

<style>
	.dashboard-container {
		min-height: 100vh;
		background: #fafafa;
	}

	.content-wrapper {
		max-width: 1400px;
		margin: 0 auto;
		padding: 2rem;
	}

	.dashboard-content {
		display: flex;
		flex-direction: column;
		gap: 2rem;
		padding-bottom: 4rem;
	}

	.welcome-section {
		display: flex;
		justify-content: space-between;
		align-items: center;
		flex-wrap: wrap;
		gap: 1rem;
	}

	.welcome-title {
		font-size: 2rem;
		font-weight: 700;
		color: #1f2937;
		margin: 0 0 0.25rem 0;
	}

	.welcome-subtitle {
		font-size: 1rem;
		color: #6b7280;
		margin: 0;
	}

	.button-content {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.button-icon {
		font-size: 1.25rem;
	}

	.stats-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
		gap: 1.5rem;
	}

	:global(.progress-card) {
		grid-column: span 2;
	}

	.stat-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 1.5rem;
	}

	.stat-title {
		font-size: 1rem;
		font-weight: 600;
		color: #1f2937;
		margin: 0 0 0.25rem 0;
	}

	.stat-subtitle {
		font-size: 0.875rem;
		color: #6b7280;
		margin: 0;
	}

	.stat-icon {
		width: 48px;
		height: 48px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.stat-icon .material-icons {
		font-size: 1.5rem;
	}

	.progress-icon {
		background: linear-gradient(135deg, #eff6ff 0%, #dbeafe 100%);
		color: #3B82F6;
	}

	.deadline-date {
		font-size: 1.75rem;
		font-weight: 700;
		color: #1f2937;
		margin: 0 0 0.5rem 0;
	}

	.deadline-days {
		font-size: 0.875rem;
		color: #6b7280;
		margin: 0;
	}

	.status-text {
		font-size: 1.5rem;
		font-weight: 600;
		color: #1f2937;
		margin: 0 0 0.5rem 0;
	}

	.status-subtitle {
		font-size: 0.875rem;
		color: #6b7280;
		margin: 0;
	}

	.section-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1.5rem;
	}

	.section-title {
		font-size: 1.5rem;
		font-weight: 700;
		color: #1f2937;
		margin: 0 0 0.25rem 0;
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.ai-icon {
		color: #A855F7;
		font-size: 1.75rem;
	}

	.section-subtitle {
		font-size: 0.875rem;
		color: #6b7280;
		margin: 0;
	}

	.interests-form {
		display: flex;
		gap: 1rem;
		align-items: flex-start;
	}

	.interests-form :global(.mdc-text-field) {
		flex: 1;
	}

	.courses-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
		gap: 1.5rem;
	}

	@media (max-width: 768px) {
		.interests-form {
			flex-direction: column;
		}

		.interests-form :global(.mdc-text-field) {
			width: 100%;
		}

		.interests-form :global(.mdc-button) {
			width: 100%;
		}
	}

	.activity-list {
		display: flex;
		flex-direction: column;
	}

	.activity-item {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.25rem 1.5rem;
		border-bottom: 1px solid #e5e7eb;
	}

	.activity-item:last-child {
		border-bottom: none;
	}

	.activity-icon-wrapper {
		width: 48px;
		height: 48px;
		border-radius: 12px;
		background: linear-gradient(135deg, #eff6ff 0%, #dbeafe 100%);
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	.activity-icon {
		font-size: 1.5rem;
		color: #3B82F6;
	}

	.activity-details {
		flex: 1;
	}

	.activity-title {
		font-size: 1rem;
		font-weight: 600;
		color: #1f2937;
		margin: 0 0 0.25rem 0;
	}

	.activity-meta {
		font-size: 0.875rem;
		color: #6b7280;
		margin: 0;
	}

	.status-badge {
		padding: 0.375rem 0.875rem;
		border-radius: 16px;
		font-size: 0.75rem;
		font-weight: 600;
	}

	.status-badge.certified {
		background: linear-gradient(135deg, #d1fae5 0%, #a7f3d0 100%);
		color: #065f46;
	}

	/* License Status by State Section */
	.license-status-section {
		margin-bottom: 2rem;
	}

	.state-list {
		display: flex;
		flex-direction: column;
	}

	.state-item {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.25rem 1.5rem;
		border-bottom: 1px solid #e5e7eb;
	}

	.state-item:last-child {
		border-bottom: none;
	}

	.state-icon-wrapper {
		width: 48px;
		height: 48px;
		border-radius: 12px;
		background: linear-gradient(135deg, #fef3c7 0%, #fde68a 100%);
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	.state-icon {
		font-size: 1.5rem;
		color: #f59e0b;
	}

	.state-details {
		flex: 1;
	}

	.state-title {
		font-size: 1rem;
		font-weight: 600;
		color: #1f2937;
		margin: 0 0 0.25rem 0;
	}

	.state-meta {
		font-size: 0.875rem;
		color: #6b7280;
		margin: 0;
	}

	.state-status {
		flex-shrink: 0;
	}

	.hours-badge {
		padding: 0.375rem 0.875rem;
		border-radius: 16px;
		font-size: 0.75rem;
		font-weight: 600;
		background: linear-gradient(135deg, #eff6ff 0%, #dbeafe 100%);
		color: #1e40af;
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 3rem 1.5rem;
		color: #9ca3af;
	}

	.empty-icon {
		font-size: 3rem;
		margin-bottom: 1rem;
		opacity: 0.5;
	}

	.empty-state p {
		margin: 0;
		font-size: 0.875rem;
	}

	/* Edit hours UI */
	.edit-hours-form {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-top: 0.25rem;
	}

	.hours-input {
		width: 80px;
		padding: 0.375rem 0.5rem;
		border: 1px solid #d1d5db;
		border-radius: 4px;
		font-size: 0.875rem;
		font-family: inherit;
	}

	.hours-input:focus {
		outline: 2px solid #3B82F6;
		outline-offset: 0;
		border-color: #3B82F6;
	}

	.edit-btn,
	.save-btn,
	.cancel-btn {
		background: transparent;
		border: none;
		cursor: pointer;
		padding: 0.25rem;
		border-radius: 4px;
		display: flex;
		align-items: center;
		justify-content: center;
		transition: background 0.2s;
	}

	.edit-btn {
		color: #6b7280;
	}

	.edit-btn:hover {
		background: #f3f4f6;
		color: #3B82F6;
	}

	.save-btn {
		color: #10b981;
	}

	.save-btn:hover:not(:disabled) {
		background: #d1fae5;
	}

	.save-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.cancel-btn {
		color: #ef4444;
	}

	.cancel-btn:hover:not(:disabled) {
		background: #fee2e2;
	}

	.cancel-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.edit-btn .material-icons,
	.save-btn .material-icons,
	.cancel-btn .material-icons {
		font-size: 1.125rem;
	}

	.state-status {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.hours-badge.editing {
		background: #fef3c7;
		color: #92400e;
	}

	/* Mobile responsive styles */
	@media (max-width: 1024px) {
		:global(.progress-card) {
			grid-column: span 1;
		}
	}

	@media (max-width: 768px) {
		.content-wrapper {
			padding: 1rem;
		}

		.welcome-title {
			font-size: 1.5rem;
		}

		.stats-grid {
			grid-template-columns: 1fr;
		}

		.courses-grid {
			grid-template-columns: 1fr;
		}

		.welcome-section {
			flex-direction: column;
			align-items: flex-start;
		}
	}
</style>
