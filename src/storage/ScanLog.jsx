import React, { useEffect, useRef } from 'react';
import "./ScanLog.css"; // Ensure this path is correct

function ScanLog({ logs }) {
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
                    <th className="right-align">Timestamp</th>
                    <th className="right-align">Resources</th>
                    <th className="right-align">Directories</th>
                    <th className="right-align">Files</th>
                    <th className="right-align">Size (GB)</th>
                    <th className="right-align">Wall Time (s)</th>
                    <th className="right-align">Resource Throughput (units/s)</th>
                    <th className="right-align">Throughput (GB/sec)</th>
                </tr>
                </thead>
                <tbody>
                {[...logs].reverse().map((log, index) => {
                    const logData = JSON.parse(log);
                    return (
                        <tr key={index}>
                            <td className="right-align">{logData.timestamp}</td>
                            <td className="right-align">{Number(logData.resources).toLocaleString()}</td>
                            <td className="right-align">{Number(logData.directories).toLocaleString()}</td>
                            <td className="right-align">{Number(logData.files).toLocaleString()}</td>
                            <td className="right-align">{formatSize(logData.size)}</td>
                            <td className="right-align">{formatWallTime(logData.wall_time_nanos)}</td>
                            <td className="right-align">{calculateResourceThroughput(Number(logData.resources), logData.wall_time_nanos)}</td>
                            <td className="right-align">{calculateThroughput(logData.size, logData.wall_time_nanos)}</td>
                        </tr>
                    );
                })}
                </tbody>
            </table>
        </div>
    );
}

export default ScanLog;