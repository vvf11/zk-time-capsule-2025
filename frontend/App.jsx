import { useState } from 'react';

function App() {
    const [unlockDate, setUnlockDate] = useState('');
    const [message, setMessage] = useState('');
    const [photo, setPhoto] = useState(null);
    const [proof, setProof] = useState(null);

    const handlePhotoChange = (e) => {
        setPhoto(e.target.files[0]);
    };

    const handleSubmit = async (e) => {
        e.preventDefault();

        try {
            const dateNum = parseInt(unlockDate.replace(/-/g, ''), 10);
            const formData = new FormData();
            formData.append('unlock_date', dateNum);
            formData.append('message', message);
            if (photo) {
                formData.append('photo_data', photo);
            }

            const response = await fetch('http://127.0.0.1:8080/generate_proof', {
                method: 'POST',
                body: formData,
            });

            if (!response.ok) {
                throw new Error(await response.text());
            }

            const data = await response.json();
            const proofBytes = new Uint8Array(Object.values(data.proof));
            setProof(proofBytes);

            const blob = new Blob([proofBytes], { type: 'application/octet-stream' });
            const url = URL.createObjectURL(blob);
            const link = document.createElement('a');
            link.href = url;
            link.download = 'proof.bin';
            link.click();
        } catch (error) {
            alert('Ошибка: ' + error.message);
        }
    };

    return (
        <div className="container">
            <h1>ZK Time Capsule</h1>
            <form onSubmit={handleSubmit}>
                <div className="form-group">
                    <label>Дата открытия:</label>
                    <input type="date" value={unlockDate} onChange={(e) => setUnlockDate(e.target.value)} required />
                </div>
                <div className="form-group">
                    <label>Сообщение:</label>
                    <textarea value={message} onChange={(e) => setMessage(e.target.value)} placeholder="Ваше секретное сообщение" required />
                </div>
                <div className="form-group">
                    <label>Прикрепить фото (опционально):</label>
                    <input type="file" accept="image/*" onChange={handlePhotoChange} />
                </div>
                <button type="submit">Запечатать капсулу</button>
            </form>
            {proof && <p className="success">Капсула запечатана! Доказательство скачано.</p>}
        </div>
    );
}

export default App;