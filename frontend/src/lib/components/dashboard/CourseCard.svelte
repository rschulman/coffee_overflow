<script lang="ts">
	import Card from '@smui/card';
	import Button from '@smui/button';
	import { STRINGS, formatString } from '$lib/constants/strings';

	interface Props {
		title: string;
		provider: string;
		topic: string;
		hours: number;
		format: string;
		price: string;
		url: string;
		rating?: number;
		aiReason: string;
	}

	let { title, provider, topic, hours, format, price, url, rating, aiReason }: Props = $props();
</script>

<Card class="course-card">
	<div class="course-header">
		<div class="course-topic-badge">{topic}</div>
		{#if rating}
			<div class="course-rating">
				<span class="material-icons rating-star">star</span>
				<span>{rating}</span>
			</div>
		{/if}
	</div>

	<h4 class="course-title">{title}</h4>
	<p class="course-provider">{provider}</p>

	<div class="ai-insight">
		<span class="material-icons insight-icon">lightbulb</span>
		<p class="insight-text">{aiReason}</p>
	</div>

	<div class="course-meta">
		<div class="meta-item">
			<span class="material-icons">schedule</span>
			<span>{hours} hours</span>
		</div>
		<div class="meta-item">
			<span class="material-icons">computer</span>
			<span>{format}</span>
		</div>
		<div class="meta-item">
			<span class="material-icons">payments</span>
			<span>{price}</span>
		</div>
	</div>

	<a href={url} target="_blank" rel="noopener noreferrer" style="text-decoration: none; width: 100%;">
		<Button
			variant="outlined"
			style="width: 100%; margin-top: 1rem; min-height: 44px;"
			aria-label={`Learn more about ${title}`}
		>
			Learn More
		</Button>
	</a>
</Card>

<style>
	:global(.course-card) {
		padding: 1.5rem !important;
		transition: transform 0.2s, box-shadow 0.2s;
		cursor: pointer;
	}

	:global(.course-card:hover) {
		transform: translateY(-4px);
		box-shadow: 0 12px 24px rgba(0, 0, 0, 0.1);
	}

	.course-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
	}

	.course-topic-badge {
		padding: 0.25rem 0.75rem;
		background: linear-gradient(135deg, #eff6ff 0%, #f3e8ff 100%);
		color: #4338ca;
		font-size: 0.75rem;
		font-weight: 600;
		border-radius: 12px;
	}

	.course-rating {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		font-size: 0.875rem;
		font-weight: 600;
		color: #1f2937;
	}

	.rating-star {
		font-size: 1rem;
		color: #fbbf24;
	}

	.course-title {
		font-size: 1.125rem;
		font-weight: 600;
		color: #1f2937;
		margin: 0 0 0.5rem 0;
		line-height: 1.4;
	}

	.course-provider {
		font-size: 0.875rem;
		color: #6b7280;
		margin: 0 0 1rem 0;
	}

	.ai-insight {
		display: flex;
		gap: 0.5rem;
		padding: 0.75rem;
		background: linear-gradient(135deg, #fefce8 0%, #fef3c7 100%);
		border-radius: 8px;
		margin-bottom: 1rem;
	}

	.insight-icon {
		font-size: 1rem;
		color: #f59e0b;
		flex-shrink: 0;
	}

	.insight-text {
		font-size: 0.8125rem;
		color: #92400e;
		margin: 0;
		line-height: 1.4;
	}

	.course-meta {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.meta-item {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.875rem;
		color: #6b7280;
	}

	.meta-item .material-icons {
		font-size: 1rem;
	}
</style>
