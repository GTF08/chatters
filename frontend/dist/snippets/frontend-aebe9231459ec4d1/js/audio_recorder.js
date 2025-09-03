let mediaRecorder;

let chunks = [];

let intervalId;

const time = 1000;

export async function startRecording(callback) {
    if (navigator.mediaDevices && navigator.mediaDevices.getUserMedia) {
        const options = {
            audio: {
                channels: 2, 
                sampleSize:16,
                channelCount: 1,
                sampleRate: 16000,
                autoGainControl: false, 
                echoCancellation: false, 
                noiseSuppression: false 
            }
        }

        const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
        mediaRecorder = new MediaRecorder(stream);
        mediaRecorder.start();
        //chunks = [];

        mediaRecorder.onstop = async function(e) {
            var audioBlob = new Blob(chunks);
            chunks = [];

            var arrayBuffer = await audioBlob.arrayBuffer();
            callback(arrayBuffer);

            mediaRecorder.start();

            setTimeout(function () {
                mediaRecorder.stop();
            }, time);

            // var arrayBuffer = await blob.arrayBuffer();
            // const uint8array = new Uint8Array(arrayBuffer);
            // callback(uint8array);
        };

        mediaRecorder.ondataavailable = async function(e) {
            chunks.push(e.data);
            //const arrayBuffer = await e.data.arrayBuffer();
            //callback(arrayBuffer);
        };
        //mediaRecorder.start();

        setTimeout(function () {
            mediaRecorder.stop();
        }, time);

        // intervalId = setInterval(function() {
        //     mediaRecorder.stop();
        //     mediaRecorder.start();
        // }, 200);
    } else {
        throw new Error("Media devices not supported");
    }
}

export function stopRecording() {
    return new Promise((resolve, reject) => {
        if (mediaRecorder) {
            // mediaRecorder.onstop = () => {
            //     // const blob = new Blob(chunks, { 'type' : 'audio/ogg; codecs=opus' });
            //     // const reader = new FileReader();
            //     // reader.onloadend = () => {
            //     //     resolve(reader.result);
            //     // };
            //     resolve();
            //     // reader.onerror = reject;
            //     // reader.readAsDataURL(blob);
            // };
            clearInterval(intervalId);
            mediaRecorder.onstop = null;
            mediaRecorder.stop();
        } else {
            reject("No recording in progress");
        }
    });
}

export function destroy() {
    if (mediaRecorder && mediaRecorder.state !== 'inactive') {
        clearInterval(intervalId);
        mediaRecorder.onstop = null;
        mediaRecorder.stop();
        mediaRecorder.ondataavailable = null;
        mediaRecorder = null;
    }
}