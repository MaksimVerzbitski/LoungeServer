document.addEventListener('DOMContentLoaded', function() {
    const messageDiv = document.getElementById('message');
    const userFields = document.getElementById('userFields');
    const readFields = document.getElementById('readFields');
    const updateFields = document.getElementById('updateFields');
    const actionSelect = document.getElementById('action');
    const performActionButton = document.getElementById('performAction');
    const userForm = document.getElementById('userForm');
    const readUserButton = document.getElementById('readUserButton');
    const readAllUsersButton = document.getElementById('readAllUsersButton');
    const userTableDiv = document.getElementById('userTable');
    const userIdField = document.getElementById('id');
    const fetchUserDataButton = document.getElementById('fetchUserDataButton');

    let handleFormSubmitCounter = 0;
    let validateFormCounter = 0;

    actionSelect.addEventListener('change', () => {
        const action = actionSelect.value;
        console.log(`Action selected: ${action}`);  // Debug
        userFields.style.display = (action === 'create') ? 'block' : 'none';
        readFields.style.display = (action === 'read' || action === 'delete' || action === 'delete_all') ? 'block' : 'none';
        updateFields.style.display = (action === 'update') ? 'block' : 'none';

        if (action !== 'read' && action !== 'read_all') {
            userTableDiv.innerHTML = '';
        }
    });

    performActionButton.addEventListener('click', () => {
        const action = actionSelect.value;
        console.log(`Perform action: ${action}`);  // Debug
        handleFormSubmit(action);
    });

    readUserButton.addEventListener('click', () => handleFormSubmit('read'));
    readAllUsersButton.addEventListener('click', () => {
        handleFormSubmit('read_all');
        userIdField.value = '';
    });

    fetchUserDataButton.addEventListener('click', fetchUserData);

    function fetchUserData() {
        const userId = document.getElementById('update_id').value;
        console.log(`Fetch user data for ID: ${userId}`);  // Debug
        if (!userId) {
            showMessage('User ID is required to fetch data.', false);
            return;
        }
        
        fetch(`/user/${userId}`)
            .then(response => response.json())
            .then(data => {
                console.log('Fetched user data:', data);  // Debug
                if (data.error) {
                    showMessage(data.error, false);
                    return;
                }

                document.getElementById('update_telegram_id').value = data.telegram_id;
                document.getElementById('update_username').value = data.username;
                document.getElementById('update_tokens').value = data.tokens;
                document.getElementById('update_referals').value = data.referals;
                document.getElementById('update_friends').value = data.friends;
                document.getElementById('update_active_chat').value = data.active_chat;
            })
            .catch(error => {
                console.error('Error fetching user data:', error);  // Debug
                showMessage('Error fetching user data. Please try again.', false);
            });
    }

    function showMessage(message, isSuccess = true) {
        console.log(`Show message: ${message}`);  // Debug
        messageDiv.innerHTML = message;
        messageDiv.style.display = 'block';
        messageDiv.style.backgroundColor = isSuccess ? '#dff0d8' : '#f2dede';
        messageDiv.style.borderColor = isSuccess ? '#4CAF50' : '#ebccd1';
        messageDiv.style.color = isSuccess ? '#3c763d' : '#a94442';
        setTimeout(() => {
            messageDiv.style.display = 'none';
        }, 10000);
    }

    function renderUsers(users) {
        let html = '<table><thead><tr>';
        html += '<th>ID</th><th>Telegram ID</th><th>Username</th><th>Tokens</th><th>Referals</th><th>Friends</th><th>Active Chat</th>';
        html += '</tr></thead><tbody>';

        users.sort((a, b) => a.id - b.id);
        users.forEach(user => {
            html += '<tr>';
            html += `<td>${user.id}</td>`;
            html += `<td>${user.telegram_id}</td>`;
            html += `<td>${user.username}</td>`;
            html += `<td>${user.tokens}</td>`;
            html += `<td>${user.referals}</td>`;
            html += `<td>${user.friends}</td>`;
            html += `<td>${user.active_chat}</td>`;
            html += '</tr>';
        });

        html += '</tbody></table>';
        console.log('Rendered users:', html);  // Debug
        return html;
    }

    function validateForm(action, data) {
        validateFormCounter++;
        console.log(`validateForm called ${validateFormCounter} times for action: ${action}`);
        const requiredFields = ['telegram_id', 'username', 'tokens', 'referals', 'friends', 'active_chat'];
        console.log(`Validate form for action: ${action}, data:`, data);  // Debug

        if (action === 'create' || action === 'update') {
            for (const field of requiredFields) {
                if (!data[field] || data[field].toString().trim() === '') {
                    console.log(`Missing field: ${field}`);  // Debug
                    return `Please fill in all required fields. Missing: ${field}`;
                }
            }
        }

        if ((action === 'read' || action === 'delete') && !data.id && action !== 'read_all' && action !== 'delete_all') {
            return 'User ID is required for this action.';
        }

        return null;
    }

    async function handleFormSubmit(action) {
        handleFormSubmitCounter++;
        console.log(`handleFormSubmit called ${handleFormSubmitCounter} times for action: ${action}`);
        
        const formData = new FormData(userForm);
        const data = {};
        formData.forEach((value, key) => {
            console.log(`Key: ${key}, Value: ${value}`);  // Debug
            data[key] = value.toString().trim();  // Ensure all values are trimmed and converted to string
        });

        console.log(`Handle form submit for action: ${action}, data:`, data);  // Debug

        if (action === 'read' || action === 'delete') {
            data.id = userIdField.value.trim();
        }

        const validationError = validateForm(action, data);

        if (validationError) {
            showMessage(validationError, false);
            return;
        }

        let url = '';
        let method = '';
        let headers = {};
        let body = null;

        switch(action) {
            case 'create':
                url = '/user';
                method = 'POST';
                headers = { 'Content-Type': 'application/x-www-form-urlencoded' };
                body = new URLSearchParams(data);
                break;
            case 'read':
                url = `/user/${data.id}`;
                method = 'GET';
                break;
            case 'read_all':
                url = `/users`;  // Adjust endpoint as needed to fetch all users
                method = 'GET';
                break;
            case 'update':
                if (!data.id) {
                    showMessage('User ID is required for update.', false);
                    return;
                }
                url = `/user/${data.id}`;
                method = 'PUT';
                headers = { 'Content-Type': 'application/x-www-form-urlencoded' };
                body = new URLSearchParams(data);
                break;
            case 'delete':
                if (!data.id) {
                    showMessage('User ID is required for delete.', false);
                    return;
                }
                url = `/user/${data.id}`;
                method = 'DELETE';
                break;
            case 'delete_all':
                url = `/users/delete_all`;
                method = 'DELETE';
                break;
        }

        try {
            /* const response = await fetch(url, {
                method: method,
                headers: headers,
                body: method === 'GET' ? null : body
            }); */

            const response = await fetch(url, {
                method: method,
                headers: headers,
                body: body
            });

            const result = await response.json();
            console.log('Response from server:', result);  // Debug

            if (response.ok) {
                if (action === 'read' || action === 'read_all') {
                    userTableDiv.innerHTML = renderUsers(action === 'read' ? [result] : result);  // Render the single user or all users
                } else {
                    showMessage(result.message, true);
                    userForm.reset();
                    // Fetch and render all users to update the table
                    if (action === 'update' || action === 'delete' || action === 'create') {
                        handleFormSubmit('read_all');
                    }
                }
            } else {
                showMessage(result.error, false);
            }
        } catch (error) {
            console.error('Error during fetch:', error);  // Debug
            showMessage('An error occurred. Please try again.', false);
        }
    }

    userForm.addEventListener('submit', (event) => {
        event.preventDefault();
        const action = actionSelect.value;
        console.log(`Form submitted for action: ${action}`);  // Debug
        handleFormSubmit(action);
    });
});






  
/* <!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>User Management</title>
    <link rel="stylesheet" href="/static/styles/styles.css">
</head>
<body>
    <div id="message"></div>
    <h1>User Management</h1>
    <div class="action-container">
        <form id="actionForm">
            <label for="action">Action:</label>
            <select id="action" name="action" required>
                <option value="create">Create User</option>
                <option value="read">Read User</option>
                <option value="update">Update User</option>
                <option value="delete">Delete User</option>
            </select>
            <button type="button" id="performAction">Perform Action</button>
        </form>
    </div>
    <form id="userForm">
        <div class="button-container">
            <button type="submit" class="submit-btn">Submit</button>
            <button type="button" id="readUserButton" class="read-btn">Read User</button>
            <button type="button" id="readAllUsersButton" class="read-btn">Read All Users</button>
        </div>
        <div id="userFields">
            <label for="telegram_id">Telegram ID:</label>
            <input type="number" id="telegram_id" name="telegram_id"><br>
            <label for="username">Username:</label>
            <input type="text" id="username" name="username"><br>
            <label for="tokens">Tokens:</label>
            <input type="number" id="tokens" name="tokens"><br>
            <label for="referals">Referals:</label>
            <input type="number" id="referals" name="referals"><br>
            <label for="friends">Friends:</label>
            <input type="number" id="friends" name="friends"><br>
            <label for="active_chat">Active Chat:</label>
            <input type="number" id="active_chat" name="active_chat"><br>
        </div>
        <div id="readFields" class="read-container" style="display:none;">
            <label for="id">User ID:</label>
            <input type="text" id="id" name="id">
        </div>
    </form>

    <div id="userTable"></div>

    <script src="/static/js/script.js"></script>
</body>
</html>
Updated CSS (styles.css)
Here is the updated CSS to align the buttons in the same row:

css
Copy code
body {
    font-family: Arial, sans-serif;
    background-color: #f4f4f4;
    margin: 0;
    padding: 20px;
    max-width: 800px;
    margin: auto;
}

h1 {
    color: #333;
}

.action-container {
    margin-bottom: 20px;
}

form {
    background-color: #fff;
    padding: 20px;
    border-radius: 8px;
    box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
    margin-bottom: 20px;
}

label {
    display: block;
    margin-bottom: 10px;
    font-weight: bold;
}

input {
    width: calc(100% - 22px);
    padding: 10px;
    margin-bottom: 20px;
    border: 1px solid #ccc;
    border-radius: 4px;
    display: block;
}

select {
    width: 100%;
    padding: 10px;
    margin-bottom: 20px;
    border: 1px solid #ccc;
    border-radius: 4px;
    background-color: #fff;
}

button {
    background-color: #4CAF50;
    color: white;
    padding: 10px 20px;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    margin-right: 10px;
    margin-bottom: 10px;
}
button:hover {
    background-color: #45a049;
}

.button-container {
    display: flex;
    align-items: center;
    margin-bottom: 20px;
}

.read-container {
    display: flex;
    align-items: center;
    margin-bottom: 20px;
}

.read-container label {
    margin-right: 10px;
}

.read-container input {
    width: auto;
    flex-grow: 1;
    margin-right: 10px;
}

#message {
    margin-top: 20px;
    padding: 10px;
    border: 2px solid #4CAF50;
    background-color: #dff0d8;
    color: #3c763d;
    font-weight: bold;
    display: none;
}

#formMessage {
    padding: 10px;
    border: 2px solid #4CAF50;
    background-color: #dff0d8;
    color: #3c763d;
    font-weight: bold;
    display: none;
}

table {
    width: 100%;
    border-collapse: collapse;
    margin-top: 20px;
}

table, th, td {
    border: 1px solid #ddd;
}

th {
    background-color: #f2f2f2;
    color: #333;
}

td {
    background-color: #fff;
    color: #333;
}

tr:nth-child(even) td {
    background-color: #f9f9f9;
}

tr:hover td {
    background-color: #f1f1f1;
}

.read-btn {
    background-color: #007BFF;
    color: white;
    border: none;
    padding: 10px 20px;
    border-radius: 4px;
    cursor: pointer;
    margin-right: 10px;
}

.read-btn:hover {
    background-color: #0056b3;
}

.submit-btn {
    margin-bottom: 20px;
}
const userFields = document.getElementById('userFields');
const actionSelect = document.getElementById('action');
const performActionButton = document.getElementById('performAction');
const userForm = document.getElementById('userForm');

actionSelect.addEventListener('change', () => {
    const action = actionSelect.value;
    userFields.style.display = (action === 'create' || action === 'update') ? 'block' : 'none';
});

performActionButton.addEventListener('click', () => {
    const action = actionSelect.value;
    handleFormSubmit(action);
});

function showMessage(message, isSuccess = true) {
    messageDiv.innerText = message;
    messageDiv.style.display = 'block';
    messageDiv.style.backgroundColor = isSuccess ? '#dff0d8' : '#f2dede';
    messageDiv.style.borderColor = isSuccess ? '#4CAF50' : '#ebccd1';
    messageDiv.style.color = isSuccess ? '#3c763d' : '#a94442';
    setTimeout(() => {
        messageDiv.style.display = 'none';
    }, 10000);
}

async function handleFormSubmit(action) {
    const formData = new FormData(userForm);
    const data = Object.fromEntries(formData);
    let url = '';
    let method = '';
    let headers = {};
    let body = null;

    switch(action) {
        case 'create':
            url = '/user';
            method = 'POST';
            headers = { 'Content-Type': 'application/x-www-form-urlencoded' };
            body = new URLSearchParams(data);
            break;
        case 'read':
            url = `/user/${data.id}`;
            method = 'GET';
            break;
        case 'update':
            url = '/user';
            method = 'PUT';
            headers = { 'Content-Type': 'application/x-www-form-urlencoded' };
            body = new URLSearchParams(data);
            break;
        case 'delete':
            url = `/user/${data.id}`;
            method = 'DELETE';
            break;
    }

    try {
        const response = await fetch(url, {
            method: method,
            headers: headers,
            body: body
        });

        const result = await response.json();

        if (response.ok) {
            showMessage(result.message);
            userForm.reset();
        } else {
            showMessage(result.error, false);
        }
    } catch (error) {
        showMessage('An error occurred. Please try again.', false);
    }
}

userForm.addEventListener('submit', (event) => {
    event.preventDefault();
    const action = actionSelect.value;
    handleFormSubmit(action);
});  */