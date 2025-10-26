<script lang="ts">
	interface Props {
		stepNumber: number;
		title: string;
		children: any;
	}

	let { stepNumber, title, children }: Props = $props();
</script>

<section class="form-section">
	<h2 class="section-title">
		<span class="step-number" aria-label="Step {stepNumber}">{stepNumber}</span>
		{title}
	</h2>
	{@render children()}
</section>

<style>
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
</style>
