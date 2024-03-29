<script setup>
import {ref, watch} from 'vue';
import {toDoItemStore} from '../stores/ToDoItem';

const emit = defineEmits(['reload-list']);

const itemStore = toDoItemStore();
const props = defineProps(["uuid", "new_status", "status"]);
const new_status = ref(props.new_status)
const status = ref(props.status)

/*
watch(() => props.status, (newStatus) => {
    status.value = newStatus;
});
 */

const handleTransition = async () => {
    await itemStore.transition(props.uuid, props.new_status);

    emit('reload-list');
};
</script>

<template>
    <div>
        <div v-if="status === 'Open'">
            <div v-if="new_status === 'In Progress'">
                <button @click="handleTransition" class="progress">
                    in progress
                </button>
            </div>
            <div v-else-if="new_status === 'Done'">
                <button @click="handleTransition" class="done">
                    done
                </button>
            </div>
        </div>
        <div v-else-if="status === 'In Progress'">
            <div v-if="new_status === 'Done'">
                <button @click="handleTransition" class="done">
                    done
                </button>
            </div>
            <div v-else-if="new_status === 'Open'">
                <button @click="handleTransition" class="open">
                    re-open
                </button>
            </div>
        </div>
        <div v-else-if="status === 'Done'">
            <div v-if="new_status === 'In Progress'">
                <button @click="handleTransition" class="progress">
                    back to work
                </button>
            </div>
            <div v-else-if="new_status === 'Open'">
                <button @click="handleTransition" class="open">
                    re-open
                </button>
            </div>
        </div>
    </div>
</template>

<style scoped>
button {
    border: none;
    color: black;
    text-align: center;
    text-decoration: none;
    display: inline-block;
    cursor: pointer;
    margin: 0.2em 0.5em;
}
button {
    float: left;
}
.progress {
    background-color: hsl(207, 100%, 37%);
}
.done {
    background-color: hsla(160, 100%, 37%, 1);
}
.open {
    background-color: hsl(52, 100%, 37%);
}
</style>