<script setup>
import CreateToDoItem from "../components/CreateToDoItem.vue";
import ToDoItems from "../components/ToDoItems.vue";
</script>

<template>
    <div class="todos">
        <h1>ToDos</h1>

        <!-- Suspense component to show to do items  -->
        <Suspense>
            <template #fallback>
                <div>loading</div>
            </template>

            <ToDoItems/>
        </Suspense>
    </div>
</template>

<style scoped>

</style>