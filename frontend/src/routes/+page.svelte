<script lang="ts">
  import { logout, start_auth, test_token } from '$lib/backend/auth.svelte';
  import { Button } from 'positron-components/components/ui';

  const login = async () => {
    let res = await start_auth();
    if (res) {
      window.location.href = res;
    }
  };

  let valid = $state();
  const test = async () => {
    valid = await test_token();
  };
</script>

<Button.Button>Tests</Button.Button>
<button onclick={login} class="text-white"> Login </button>
<button onclick={logout} class="text-white"> Logout </button>
<button onclick={test} class="text-white"> Test </button>
<p class="text-white">Valid: {valid}</p>
