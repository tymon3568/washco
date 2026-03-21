<script lang="ts">
	import type { Snippet } from 'svelte';

	let { open = $bindable(false), title, children }: {
		open: boolean;
		title?: string;
		children: Snippet;
	} = $props();

	function close() {
		open = false;
	}

	function onKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') close();
	}
</script>

{#if open}
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div
		class="backdrop-blur-overlay fixed inset-0 z-[90] bg-black/40"
		onclick={close}
		onkeydown={onKeydown}
		role="presentation"
	></div>
	<div
		class="fixed inset-x-0 bottom-0 z-[91] mx-auto max-w-lg rounded-t-2xl bg-card shadow-xl"
		role="dialog"
		aria-modal="true"
		aria-label={title}
	>
		<div class="flex items-center justify-center pt-3">
			<div class="h-1 w-10 rounded-full bg-muted-foreground/20"></div>
		</div>
		{#if title}
			<div class="flex items-center justify-between border-b border-border px-5 pb-3 pt-2">
				<h2 class="text-lg font-semibold">{title}</h2>
				<button onclick={close} class="rounded-full p-1 text-muted-foreground hover:bg-muted" aria-label="Dong">
					<svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
						<path d="M18 6 6 18M6 6l12 12" />
					</svg>
				</button>
			</div>
		{/if}
		<div class="max-h-[70vh] overflow-y-auto p-5 pb-safe">
			{@render children()}
		</div>
	</div>
{/if}
