<script setup>
</script>
<script  lang="ts">
import { defineComponent } from "vue";
import { toDoItemsStore } from '../stores/ToDoItems'

export default defineComponent({
  async setup() {
    const itemStore = toDoItemsStore()

    // Updating
    await itemStore.update()

    return {
      itemStore
    }
  }
})
</script>

<template>
  <div>

    <hr>

    <div v-if="itemStore.openItemsCount === 1">
      <h2>You have {{ itemStore.openItemsCount }} open item</h2>
    </div>
    <div v-else>
      <h2>You have {{ itemStore.openItemsCount }} open items</h2>
    </div>

    <!-- list to do items -->
    <ul>
      <li v-for="item in itemStore.openItems">{{ item.title }}</li>
    </ul>

    <hr>

    <div v-if="itemStore.doneItemsCount === 1">
      <h2>You have {{ itemStore.doneItemsCount }} done item</h2>
    </div>
    <div v-else>
      <h2>You have {{ itemStore.doneItemsCount }} done items</h2>
    </div>
  </div>
</template>

<style scoped>
  hr {
    margin-bottom: 20px;
    margin-top: 40px;
  }
</style>