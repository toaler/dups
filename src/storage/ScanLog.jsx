import React, {useEffect, useRef} from 'react';

function ScanLog({ logs }) {

    const endOfLogsRef = useRef(null);

    useEffect(() => {
        endOfLogsRef.current?.scrollIntoView({behavior: 'smooth'});
    }, [logs]); // Dependency array, this effect runs when `logs` changes

    return (
        <div className="log-container" style={{ height: '300px', overflowY: 'auto' }}>
            {logs.map((log, index) => (<div key={index}>{log}</div>))}
            <div ref={endOfLogsRef} /> {/* Invisible div at the end of your logs */}
        </div>
    );
};

export default ScanLog;