<script>
import Footer from "$lib/Footer.svelte";
import Header from "$lib/Header.svelte";

import "../app.css";
</script>
  
<div class="flex flex-col h-full">
  <Header />
  <div class="flex-grow">
    <slot />
  </div>
  <Footer />
</div>
  