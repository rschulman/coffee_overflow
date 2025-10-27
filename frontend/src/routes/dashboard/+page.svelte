<script lang="ts">
	import Card from '@smui/card';
	import { userData } from '$lib/stores/user';
	import { RECOMMENDED_COURSES, RECENT_COURSES } from '$lib/data/mockCourses';
	import { STRINGS, formatString } from '$lib/constants/strings';
	import type { Course } from '$lib/types';
	import AppBar from '$lib/components/dashboard/AppBar.svelte';
	import StatCard from '$lib/components/dashboard/StatCard.svelte';
	import ProgressCircle from '$lib/components/dashboard/ProgressCircle.svelte';
	import CourseCard from '$lib/components/dashboard/CourseCard.svelte';
	import GradientButton from '$lib/components/shared/GradientButton.svelte';

	// Get user data from store
	const user = $derived($userData);
	const userName = $derived(user.fullName.split(' ')[0] || 'User');
	const profession = $derived(user.profession);
	const state = $derived(user.state);
	const hoursRequired = $derived(user.hoursRequired);
	const hoursCompleted = $derived(user.hoursCompleted);
	const renewalDate = $derived(user.renewalDate);
	const selectedTopics = $derived(user.selectedTopics);

	const progressPercentage = $derived(
		hoursRequired > 0 ? (hoursCompleted / hoursRequired) * 100 : 0
	);
	const daysUntilRenewal = $derived(
		renewalDate ? Math.ceil((new Date(renewalDate).getTime() - new Date().getTime()) / (1000 * 60 * 60 * 24)) : 0
	);

	// Filter courses based on user's selected topics
	const recommendedCourses = $derived(
		selectedTopics.length > 0
			? RECOMMENDED_COURSES.filter(course => selectedTopics.includes(course.topic))
			: RECOMMENDED_COURSES
	);

	function formatDate(dateString: string) {
		return new Date(dateString).toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' });
	}

	function handleCertificationSubmit() {
		alert(formatString(STRINGS.dashboard.certificationAlert, { state }));
	}

	function handleEnrollClick(course: Course) {
		alert(formatString(STRINGS.dashboard.enrollmentAlert, { course: course.title }));
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
					<p class="welcome-subtitle">{profession} • {state}</p>
				</div>
				<GradientButton
					onclick={handleCertificationSubmit}
					ariaLabel={STRINGS.aria.submitCertToStateBar}
				>
					<div class="button-content">
						<span class="material-icons button-icon">send</span>
						<span>{STRINGS.dashboard.submitCertification}</span>
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
					<ProgressCircle {hoursCompleted} {hoursRequired} />
				</Card>

				<!-- Deadline Card -->
				<StatCard
					title={STRINGS.dashboard.nextRenewal}
					subtitle={STRINGS.dashboard.licenseExpiration}
					icon="event"
					iconBgColor="linear-gradient(135deg, #fef3c7 0%, #fde68a 100%)"
				>
					<p class="deadline-date">{renewalDate ? formatDate(renewalDate) : STRINGS.dashboard.notSet}</p>
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

			<!-- AI Recommendations Section -->
			<section class="recommendations-section">
				<div class="section-header">
					<div>
						<h3 class="section-title">
							<span class="material-icons ai-icon">auto_awesome</span>
							{STRINGS.dashboard.aiRecommendations}
						</h3>
						<p class="section-subtitle">
							{selectedTopics.length > 0
								? formatString(STRINGS.dashboard.personalizedCourses, { topics: selectedTopics.join(', ') })
								: STRINGS.dashboard.personalizedCoursesDefault}
						</p>
					</div>
				</div>

				<div class="courses-grid">
					{#each recommendedCourses as course}
						<CourseCard
							title={course.title}
							provider={course.provider}
							topic={course.topic}
							hours={course.hours}
							format={course.format}
							price={course.price}
							rating={course.rating}
							aiReason={course.aiReason}
							onEnroll={() => handleEnrollClick(course)}
						/>
					{/each}
				</div>
			</section>

			<!-- Recent Activity Section -->
			<section class="activity-section">
				<h3 class="section-title">{STRINGS.dashboard.recentActivity}</h3>
				<Card style="padding: 0; overflow: hidden;">
					<div class="activity-list">
						{#each RECENT_COURSES as course}
							<div class="activity-item">
								<div class="activity-icon-wrapper">
									<span class="material-icons activity-icon">school</span>
								</div>
								<div class="activity-details">
									<h4 class="activity-title">{course.title}</h4>
									<p class="activity-meta">{formatString(STRINGS.dashboard.completedOn, { date: formatDate(course.completedDate), hours: course.hours })}</p>
								</div>
								<div class="activity-status">
									<span class="status-badge certified">{course.status}</span>
								</div>
							</div>
						{/each}
					</div>
				</Card>
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

	.courses-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
		gap: 1.5rem;
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
