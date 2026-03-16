import { STORAGE_KEY } from './../lib/constants';

const safeGet = (key) => {
    if (typeof window === 'undefined') return [];
    try {
        return JSON.parse(localStorage.getItem(key)) || [];
    } catch {
        return [];
    }
};

const safeSet = (key, value) => {
    if (typeof window === 'undefined') return;
    try {
        localStorage.setItem(key, JSON.stringify(value));
    } catch (e) {
        console.error('localStorage set failed', e);
    }
};

export const addOrUpdateByUuid = (key = STORAGE_KEY, item) => {
    console.log(item)
    if (typeof window === 'undefined' || !item) return [];
    const arr = safeGet(key);

    if (item.uuid) {
        const idx = arr.findIndex((x) => x && x.uuid === item.uuid);
        if (idx > -1) {
            arr[idx] = { ...arr[idx], ...item };
        } else {
            arr.push(item);
        }
    } else {
        arr.push(item);
    }

    safeSet(key, arr);
    return arr;
};

/**
 * Retrieve an object by storage key and uuid.
 * Returns the matched object or an empty object {} if not found.
 */
export const getByKeyAndUuid = (key = STORAGE_KEY, uuid) => {
    if (typeof window === 'undefined' || !uuid) return {};
    const arr = safeGet(key);
    const found = arr.find((x) => x && x.uuid === uuid);
    return found ? found : {};
};

