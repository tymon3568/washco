<script lang="ts">
	let { value = $bindable(5), readonly = false, size = 'md' }: {
		value: number;
		readonly?: boolean;
		size?: 'sm' | 'md' | 'lg';
	} = $props();

	const sizes = { sm: 'text-lg', md: 'text-2xl', lg: 'text-3xl' };
</script>

<div class="flex gap-1" role={readonly ? 'img' : 'radiogroup'} aria-label="Danh gia {value}/5">
	{#each [1, 2, 3, 4, 5] as star (star)}
		{#if readonly}
			<span class="{sizes[size]} {star <= value ? 'text-yellow-400' : 'text-muted-foreground/20'}">
				&#9733;
			</span>
		{:else}
			<button
				type="button"
				onclick={() => { value = star; }}
				class="{sizes[size]} transition-transform active:scale-110 {star <= value ? 'text-yellow-400' : 'text-muted-foreground/20'}"
				aria-label="{star} sao"
			>
				&#9733;
			</button>
		{/if}
	{/each}
</div>
