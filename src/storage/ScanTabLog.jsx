import React, { useEffect, useRef } from 'react';
import "./ScanTabLog.css"; // Ensure this path is correct

function ScanTabLog({ logs }) {
    const tableRef = useRef(null);

    useEffect(() => {
        tableRef.current?.scrollIntoView({ behavior: 'smooth' });
    }, [logs]);

    // Helper function to convert bytes to gigabytes
    const formatSize = (sizeInBytes) => {
        const sizeInGB = sizeInBytes / (1024 ** 3); // Convert bytes to GB
        return sizeInGB.toLocaleString(undefined, { minimumFractionDigits: 1, maximumFractionDigits: 1 });
    };

    // Updated helper function to convert nanoseconds to seconds
    const formatWallTime = (wallTimeNanos) => {
        const totalSeconds = wallTimeNanos / 1_000_000_000; // Convert nanos to seconds
        return totalSeconds.toLocaleString('default', { minimumFractionDigits: 3, maximumFractionDigits: 3 });
    };

    // Helper function to calculate throughput in GB/sec
    const calculateThroughput = (sizeInBytes, wallTimeNanos) => {
        if (wallTimeNanos === 0) return "N/A"; // Avoid division by zero
        const sizeInGB = sizeInBytes / (1024 ** 3); // Convert bytes to GB
        const timeInSeconds = wallTimeNanos / 1_000_000_000; // Convert nanos to seconds
        const throughput = sizeInGB / timeInSeconds;
        return throughput.toLocaleString('default', { minimumFractionDigits: 2, maximumFractionDigits: 2 });
    };

    // Assuming resource throughput is a function of resources and time
    const calculateResourceThroughput = (resources, wallTimeNanos) => {
        if (wallTimeNanos === 0) return "N/A"; // Prevent division by zero
        const timeInSeconds = wallTimeNanos / 1_000_000_000; // Convert nanos to seconds
        const resourceThroughput = resources / timeInSeconds;
        return resourceThroughput.toLocaleString('default', { minimumFractionDigits: 2, maximumFractionDigits: 2 });
    };

    return (
        <div className="log-container" style={{ height: '300px', overflowY: 'auto' }}>
            <table ref={tableRef}>
                <thead>
                <tr>
                    <th>Timestamp</th>
                    <th>Resources</th>
                    <th>Directories</th>
                    <th>Files</th>
                    <th>Size (GB)</th>
                    <th>Wall Time (s)</th>
                    <th>Resource Throughput (resources/s)</th>
                    <th>Throughput (GB/sec)</th>
                </tr>
                </thead>
                <tbody>
                {[...logs].reverse().map((log, index) => {
                    const logData = JSON.parse(log);
                    return (
                        <tr key={index}>
                            <td>{logData.timestamp}</td>
                            <td>{Number(logData.resources).toLocaleString()}</td>
                            <td>{Number(logData.directories).toLocaleString()}</td>
                            <td>{Number(logData.files).toLocaleString()}</td>
                            <td>{formatSize(logData.size)}</td>
                            <td>{formatWallTime(logData.wall_time_nanos)}</td>
                            <td>{calculateResourceThroughput(Number(logData.resources), logData.wall_time_nanos)}</td>
                            <td>{calculateThroughput(logData.size, logData.wall_time_nanos)}</td>
                        </tr>
                    );
                })}
                </tbody>
            </table>
        </div>
    );
}

export default ScanTabLog;