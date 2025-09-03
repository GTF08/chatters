

export async function play_audio(byteArray) {
    const options = {
        sampleRate: 16000
    }
    const audioContext = new AudioContext(options);
  
    // console.log(uint8array.length);
    //const blob = event.data instanceof Blob ? event.data : new Blob([event.data]);
    const blob = new Blob([byteArray], { type: 'audio/wav' });
    // const url = URL.createObjectURL(blob);
    // console.log(url);
    let arrayBuffer = await blob.arrayBuffer();
    // console.log(arrayBuffer);
    // console.log(blob.size)
    // // const audio = new Audio(url);
    // // await audio.play();
    // // // Decode audio data
    const audioBuffer = await audioContext.decodeAudioData(await blob.arrayBuffer());
    
    // // // Create a buffer source and play immediately
    const source = audioContext.createBufferSource();
    source.buffer = audioBuffer;
    source.connect(audioContext.destination);
    source.start();

    var audio = new Audio(byteArray);
    audio.play();
}