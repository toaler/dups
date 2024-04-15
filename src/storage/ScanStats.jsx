import React from 'react';
import styled from "styled-components";
import "./ScanStats.css";

const formatElapsedTime = (elapsedTime) => {
    const hours = Math.floor(elapsedTime / 3600000);
    const minutes = Math.floor((elapsedTime % 3600000) / 60000);
    const seconds = Math.floor((elapsedTime % 60000) / 1000);
    const milliseconds = elapsedTime % 1000;

    return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}.${milliseconds.toString().padStart(3, '0')}`;
};

function ScanStats({status, elapsedTime, resources, directories, files, size}) {
    const sizeInGB = (size / 1073741824).toFixed(2); // Convert size from bytes to GB
    const throughput = elapsedTime > 0 ? (size / 1073741824 / (elapsedTime / 1000)).toFixed(2) : 0; // Calculate throughput in GB/sec
    const resourcesPerSecond = elapsedTime > 0 ? (resources / (elapsedTime / 1000)).toFixed(2) : 0; // Calculate resources per second

    return <ScanStatsContainer>
        <div className="flex-container">
            <div className="flex-row">
                <div className="flex-item">Status</div>
                <div className="flex-item">{status}</div>
            </div>
            <div className="flex-row">
                <div className="flex-item">Elapsed</div>
                <div className="flex-item">{formatElapsedTime(elapsedTime)}</div>
            </div>
            <div className="flex-row">
                <div className="flex-item">Resources</div>
                <div className="flex-item">{Number(resources).toLocaleString()}</div>
            </div>
            <div className="flex-row">
                <div className="flex-item">Directories</div>
                <div className="flex-item">{Number(directories).toLocaleString()}</div>
            </div>
            <div className="flex-row">
                <div className="flex-item">Files</div>
                <div className="flex-item">{Number(files).toLocaleString()}</div>
            </div>
            <div className="flex-row">
                <div className="flex-item">Size (GB)</div>
                <div className="flex-item">{sizeInGB}</div>
            </div>
            <div className="flex-row">
                <div className="flex-item">Resources/sec</div>
                <div className="flex-item">{Number(resourcesPerSecond).toLocaleString()}</div>
            </div>
            <div className="flex-row">
                <div className="flex-item">Throughput (GB/sec)</div>
                <div className="flex-item">{throughput}</div>
            </div>
        </div>
    </ScanStatsContainer>
}

export default ScanStats;

const ScanStatsContainer = styled.div`
    display: flex;
    line-height: 24px;
    font-weight: 400;
    color: #FFFFFF;
    overflow-y: hidden;
`;
