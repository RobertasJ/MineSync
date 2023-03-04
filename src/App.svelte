<script lang="ts">
  import FolderList from "./model/FolderList/FolderList.svelte";
  import EditPanel from "./model/EditPanel/EditPanel.svelte";
  import Notifications from "./model/notifications/notfications.svelte";
</script>

<main class="container text-white">
  <div>
    <FolderList />
    <EditPanel />
    <Notifications />
  </div>
</main>

